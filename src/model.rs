use super::*;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Language {
  pub id: usize,
  pub name: String,
  pub is_archived: Option<bool>,
  pub source_file: Option<String>,
  pub compile_cmd: Option<String>,
  pub run_cmd: Option<String>,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Status {
  pub id: usize,
  pub description: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct About {
  pub version: String,
  pub homepage: String,
  pub source_code: String,
  pub maintainer: String,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq)]
pub struct Worker {
  pub queue: String,
  pub size: usize,
  pub available: usize,
  pub idle: usize,
  pub working: usize,
  pub paused: usize,
  pub failed: usize,
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Submission {
  /// Program’s source code.
  pub source_code: String,

  /// The submission language identifier.
  pub language_id: usize,

  /// Options for the compiler (i.e. compiler flags).
  pub compiler_options: Option<String>,

  /// Command line arguments for the program.
  pub command_line_arguments: Option<String>,

  /// Standard input for the program.
  pub stdin: Option<String>,

  /// Expected output of the program. Used when you want to compare with the
  /// standard output.
  pub expected_output: Option<String>,

  /// Default runtime limit for every program. Time in which the OS assigns
  /// the processor to different tasks is not counted.
  pub cpu_time_limit: Option<f64>,

  /// When a time limit is exceeded, wait for extra time, before killing the
  /// program. This has the advantage that the real execution time is
  /// reported, even though it slightly exceeds the limit.
  pub cpu_extra_time: Option<f64>,

  /// Limit wall-clock time in seconds. Decimal numbers are allowed. This clock
  /// measures the time from the start of the program to its exit, so it does
  /// not stop when the program has lost the CPU or when it is waiting for an
  /// external event. We recommend to use cpu_time_limit as the main limit, but
  /// set wall_time_limit to a much higher value as a precaution against
  /// sleeping programs.
  pub wall_time_limit: Option<f64>,

  /// Limit address space of the program.
  pub memory_limit: Option<f64>,

  /// Limit process stack.
  pub stack_limit: Option<usize>,

  /// Maximum number of processes and/or threads program can create.
  pub max_processes_and_or_threads: Option<usize>,

  /// If true then cpu_time_limit will be used as per process and thread.
  pub enable_per_process_and_thread_time_limit: Option<bool>,

  /// If true then memory_limit will be used as per process and thread.
  pub enable_per_process_and_thread_memory_limit: Option<bool>,

  /// Limit file size created or modified by the program.
  pub max_file_size: Option<usize>,

  /// If true standard error will be redirected to standard output.
  pub redirect_stderr_to_stdout: Option<bool>,

  /// If true program will have network access.
  pub enable_network: Option<bool>,

  /// Run each program number_of_runs times and take average of time and
  /// memory.
  pub number_of_runs: Option<usize>,

  /// Additional files that should be available alongside the source
  /// code. Value of this string should represent the content of a .zip
  /// that contains additional files. This attribute is required for multi-file
  /// programs.
  pub additional_files: Option<String>,

  /// URL on which Judge0 will issue PUT request with the submission in a
  /// request body after submission has been done.
  pub callback_url: Option<String>,

  /// Standard output of the program after execution.
  pub stdout: Option<String>,

  /// Standard error of the program after execution.
  pub stderr: Option<String>,

  /// Compiler output after compilation.
  pub compile_output: Option<String>,

  /// If submission status is Internal Error then this message comes from
  /// Judge0 itself, otherwise this is status message from isolate.
  pub message: Option<String>,

  /// The program’s exit code.
  pub exit_code: Option<i64>,

  /// Signal code that the program received before exiting.
  pub exit_signal: Option<i64>,

  /// Submission status.
  pub status: Option<Status>,

  /// Date and time when submission was created.
  pub created_at: Option<DateTime<Utc>>,

  /// Date and time when submission was processed.
  pub finished_at: Option<DateTime<Utc>>,

  /// Unique submission token which can be used to get a specific submission.
  pub token: Option<String>,

  /// Program’s run time.
  pub time: Option<f64>,

  /// Program’s wall time. Will be greater or equal to time.
  pub wall_time: Option<f64>,

  /// Memory used by the program after execution.
  pub memory: Option<f64>,
}
