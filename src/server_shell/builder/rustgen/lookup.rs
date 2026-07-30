//SPDX:MIT

//! Tools for generating rust code to look up values within the runtime environment.

use super::super::{collect, errors};
use super::{helpers, names, types};
use crate::server_shell::lls;
use crate::shell_lib::structure;

pub const ANY_COMMAND_NAME: &'static str = "$command";

/// Generate the lookup segment rust code.
/// This creates a piece of code that you can inject within another statement.
/// In the case of errors, this will collect as much information as possible and
/// return an as-valid-as-possible rust construction.  This allows the processing
/// to continue to collect issues without requiring the end-user to perform a
/// second pass.
///
/// The 'runtime_ref' has the context-specific lookup to the runtime structure's
/// instance variable.  This should be an `Arc<Runtime>` value.
pub fn generate_lookup<'a, 'b, 'c, 'd, 'e, 'f, 'g>(
    source: &'a lls::model::Source,
    runtime_ref: &'b String,
    job_ref: &'c String,
    state_parameter: &'d String,
    expected_type: &'e structure::meta::ValueType,
    optional: bool,
    issues: &'f errors::ScriptIssues,
    collector: &'g collect::Collector,
) -> String {
    if ANY_COMMAND_NAME == job_ref.as_str() {
        return generate_all_command_lookup(
            source,
            runtime_ref,
            state_parameter,
            expected_type,
            optional,
            issues,
            collector,
        );
    }

    let job = match issues.add_result(collector.get_job_checked(job_ref, source)) {
        Some(j) => j,
        None => {
            // Job doesn't exist.  There's nothing else to do here.
            return format!(
                "panic!({})",
                helpers::as_rust_str(&format!("no such job: {}", job_ref))
            );
        }
    };
    if job.is_cmd {
        return generate_one_command_lookup(
            source,
            runtime_ref,
            job_ref,
            state_parameter,
            expected_type,
            optional,
            issues,
            collector,
        );
    }

    // One job.
    todo!()
}

/// Generate the abstract command lookup.
/// This ensures that the requested state parameter exists for all the commands.
/// While not a compile-time error if not all commands provide the state, it
/// can lead to runtime problems if the script author didn't take care with the
/// thread usage of jobs.
fn generate_all_command_lookup<'a, 'b, 'c, 'd, 'e, 'f>(
    source: &'a lls::model::Source,
    runtime_ref: &'b String,
    state_parameter: &'c String,
    expected_type: &'d structure::meta::ValueType,
    optional: bool,
    issues: &'e errors::ScriptIssues,
    collector: &'f collect::Collector,
) -> String {
    let mut ret = format!(
        "match {}.{}() {{",
        runtime_ref,
        names::COMMAND_RUNTIME_STRUCT_GETTER_NAME,
    );

    for (name, job_ref, _) in collector.ordered_commands() {
        // Generate a match arm for each job.
        ret.push_str(
            format!(
                "{}::{}(j) => {{",
                helpers::qualify_name(
                    &names::runtime_mod(),
                    &names::command_runtime_state_enum_struct_name(),
                ),
                names::command_runtime_enum_name(job_ref),
            )
            .as_str(),
        );
        match issues.add_result(collector.get_job_state_field(
            source,
            &name,
            state_parameter,
            &Some(expected_type.clone()),
        )) {
            Some((_, Some(field))) => {
                if types::is_type_match(
                    expected_type.clone(),
                    optional,
                    Some(field.value_type.clone()),
                    None,
                ) {
                    // All good.
                    ret.push_str(format!("j.{}", state_parameter).as_str());
                } else {
                    // TODO add this as a ScriptIssue?
                    ret.push_str(&helpers::return_script_exit(
                        source,
                        190,
                        format!(
                            "(script error) incompatible command {} field {}",
                            name, state_parameter
                        ),
                    ));
                }
            }
            Some(_) => {
                // Can't determine the type right now.  Assume it's valid.
                // However, if it's not valid, this will cause a compilation error when it should
                // be a runtime error.  Don't know the right expectd behavior here.
                ret.push_str(format!("j.{}", state_parameter).as_str());
            }
            None => {
                // Invalid setup.
                // This creates a still-buildable code, but the above 'get_job_state_field' reported
                // an error.
                ret.push_str(&helpers::return_script_exit(
                    source,
                    190,
                    format!(
                        "(script error) incompatible command {} field {}",
                        name, state_parameter
                    ),
                ));
            }
        }
        ret.push('}');
    }

    ret.push('}');
    ret
}

fn generate_one_command_lookup<'a, 'b, 'c, 'd, 'e, 'f, 'g>(
    source: &'a lls::model::Source,
    runtime_ref: &'b String,
    job_name: &'c String,
    state_parameter: &'d String,
    expected_type: &'e structure::meta::ValueType,
    optional: bool,
    issues: &'f errors::ScriptIssues,
    collector: &'g collect::Collector,
) -> String {
    let job_ref = match issues.add_result(collector.get_job_state_field(
        source,
        &job_name,
        state_parameter,
        &Some(expected_type.clone()),
    )) {
        Some((job_ref, Some(field))) => {
            if types::is_type_match(
                expected_type.clone(),
                optional,
                Some(field.value_type.clone()),
                None,
            ) {
                // All good.
                job_ref
            } else {
                issues.add_err(errors::BuilderError::FieldTypeMismatch(
                    errors::ErrorDetails {
                        source: source.into(),
                        message: state_parameter.clone(),
                        related: Vec::new(),
                    },
                ));
                return format!("panic!(\"\")");
            }
        }
        Some((job_ref, _)) => {
            // No field found, so can't determine the type yet.  Assume it's valid.
            job_ref
        }
        None => {
            // Error
            return format!("panic!(\"\")");
        }
    };
    let mut ret = format!(
        "match {}.{}() {{ {}::{}(j) => j.{},",
        runtime_ref,
        names::COMMAND_RUNTIME_STRUCT_GETTER_NAME,
        helpers::qualify_name(
            &names::runtime_mod(),
            &names::command_runtime_state_enum_struct_name(),
        ),
        names::command_runtime_enum_name(job_ref),
        state_parameter,
    );

    ret.push_str("_ => {");
    ret.push_str(&helpers::return_script_exit(
        source,
        191,
        format!("(script error) expected current command {}", job_name),
    ));
    ret.push_str("}}");
    ret
}
