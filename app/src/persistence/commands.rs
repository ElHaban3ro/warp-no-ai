use anyhow::Result;
use diesel::{sqlite::SqliteConnection, ExpressionMethods, QueryDsl, RunQueryDsl};

use crate::terminal::event::UserBlockCompleted;

/// Exit codes considered "successful" — kept in sync with
/// `warp_core::command::ExitCode::was_successful` (0, 130 SIGINT, 141 SIGPIPE,
/// Windows STATUS_CONTROL_C_EXIT). Used to exclude failed historical commands
/// from autosuggestion / next-command predictions.
const SUCCESSFUL_EXIT_CODES: &[i32] = &[0, 130, 141, -1073741510];

/// Returns the command that was run right after `command`
/// in the same session, if any.
///
/// Skips successors that exited with a non-successful status so the
/// autosuggestion engine never proposes a known-failed command as the next
/// step.
pub fn get_next_command(
    conn: &mut SqliteConnection,
    command: &super::model::Command,
) -> Result<super::model::Command> {
    let next_command = super::schema::commands::dsl::commands
        .filter(super::schema::commands::columns::id.gt(command.id))
        .filter(super::schema::commands::columns::session_id.eq(&command.session_id))
        // Skip any empty blocks
        .filter(super::schema::commands::columns::command.ne(""))
        // Skip commands that haven't reported an exit code yet (still running
        // or crashed before completion) and any that ended in failure.
        .filter(super::schema::commands::columns::exit_code.eq_any(SUCCESSFUL_EXIT_CODES))
        .order(super::schema::commands::columns::id.asc())
        .limit(1)
        .first::<super::model::Command>(conn)?;
    Ok(next_command)
}

/// Returns the commands that were run right before `command`
/// in the same session, if any. They are ordered from oldest to newest.
pub fn get_previous_commands(
    conn: &mut SqliteConnection,
    command: &super::model::Command,
    num_commands: usize,
) -> Result<Vec<super::model::Command>> {
    let previous_commands = super::schema::commands::dsl::commands
        .filter(super::schema::commands::columns::id.lt(command.id))
        .filter(super::schema::commands::columns::session_id.eq(&command.session_id))
        // Skip any empty blocks
        .filter(super::schema::commands::columns::command.ne(""))
        .order(super::schema::commands::columns::id.desc())
        .limit(num_commands as i64)
        .load::<super::model::Command>(conn)?;
    Ok(previous_commands.into_iter().rev().collect())
}

/// Gets the last num_commands times the same command was run in a similar context
/// (same pwd, exit code, shell, hostname), from newest to oldest.
pub fn get_same_commands_from_history(
    conn: &mut SqliteConnection,
    completed_block: &UserBlockCompleted,
    num_commands: usize,
) -> Result<Vec<super::model::Command>> {
    let shell_host = completed_block.serialized_block.shell_host.as_ref();
    let commands = super::schema::commands::dsl::commands
        .filter(super::schema::commands::columns::command.eq(&completed_block.command))
        .filter(super::schema::commands::columns::pwd.eq(&completed_block.serialized_block.pwd))
        .filter(
            super::schema::commands::columns::exit_code
                .eq(completed_block.serialized_block.exit_code.value()),
        )
        .filter(
            super::schema::commands::columns::shell
                .eq(shell_host.map(|host| host.shell_type.name())),
        )
        .filter(
            super::schema::commands::columns::hostname.eq(shell_host.map(|host| &host.hostname)),
        )
        // Get newest to oldest commands.
        .order(super::schema::commands::columns::id.desc())
        .limit(num_commands as i64)
        .load::<super::model::Command>(conn)?;

    Ok(commands)
}
