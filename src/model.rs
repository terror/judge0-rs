use super::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Language {
  pub id: usize,
  pub name: String,
  pub is_archived: Option<bool>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Status {
  pub id: usize,
  pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct About {
  pub version: String,
  pub homepage: String,
  pub source_code: String,
  pub maintainer: String,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Submission {
  /// Program’s source code.
  source_code: String,
  /// The submission language identifier.
  language_id: usize,
  /// Options for the compiler (i.e. compiler flags).
  compiler_options: Option<String>,
  /// Command line arguments for the program.
  command_line_arguments: Option<String>,
  /// Standard input for the program.
  stdin: Option<String>,
  /// Expected output of the program. Used when you want to compare with the
  /// standard output.
  expected_output: Option<String>,
  /// Default runtime limit for every program. Time in which the OS assigns
  /// the processor to different tasks is not counted.
  cpu_time_limit: Option<f64>,
  /// When a time limit is exceeded, wait for extra time, before killing the
  /// program. This has the advantage that the real execution time is
  /// reported, even though it slightly exceeds the limit.
  cpu_extra_time: Option<f64>,
  /// Limit wall-clock time in seconds. Decimal numbers are allowed. This clock
  /// measures the time from the start of the program to its exit, so it does
  /// not stop when the program has lost the CPU or when it is waiting for an
  /// external event. We recommend to use cpu_time_limit as the main limit, but
  /// set wall_time_limit to a much higher value as a precaution against
  /// sleeping programs.
  wall_time_limit: Option<f64>,
  /// Limit address space of the program.
  memory_limit: Option<f64>,
  /// Limit process stack.
  stack_limit: Option<usize>,
  /// Maximum number of processes and/or threads program can create.
  max_processes_and_or_threads: Option<usize>,
  /// If true then cpu_time_limit will be used as per process and thread.
  enable_per_process_and_thread_time_limit: Option<bool>,
  /// If true then memory_limit will be used as per process and thread.
  enable_per_process_and_thread_memory_limit: Option<bool>,
  /// Limit file size created or modified by the program.
  max_file_size: Option<usize>,
  /// If true standard error will be redirected to standard output.
  redirect_stderr_to_stdout: Option<bool>,
  /// If true program will have network access.
  enable_network: Option<bool>,
  /// Run each program number_of_runs times and take average of time and
  /// memory.
  number_of_runs: Option<usize>,
  /// Additional files that should be available alongside the source
  /// code. Value of this string should represent the content of a .zip
  /// that contains additional files. This attribute is required for multi-file
  /// programs.
  additional_files: Option<String>,
  /// URL on which Judge0 will issue PUT request with the submission in a
  /// request body after submission has been done.
  callback_url: Option<String>,
  /// Standard output of the program after execution.
  stdout: Option<String>,
  /// Standard error of the program after execution.
  stderr: Option<String>,
  /// Compiler output after compilation.
  compile_output: Option<String>,
  /// If submission status is Internal Error then this message comes from
  /// Judge0 itself, otherwise this is status message from isolate.
  message: Option<String>,
  /// The program’s exit code.
  exit_code: Option<i64>,
  /// Signal code that the program received before exiting.
  exit_signal: Option<i64>,
  /// Submission status.
  status: Option<Status>,
  /// Date and time when submission was created.
  created_at: Option<DateTime<Utc>>,
  /// Date and time when submission was processed.
  finished_at: Option<DateTime<Utc>>,
  /// Unique submission token which can be used to get a specific submission.
  token: Option<String>,
  /// Program’s run time.
  time: Option<f64>,
  /// Program’s wall time. Will be greater or equal to time.
  wall_time: Option<f64>,
  /// Memory used by the program after execution.
  memory: Option<f64>,
}
