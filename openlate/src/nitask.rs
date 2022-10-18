use crate::nichannel::nivirtchan::VirtualChannel;
use crate::nicommon::nierr::daqmx_check;
use anyhow::Result;
use nidaqmx_sys::*;
use std::collections::HashMap;
use std::ffi::{c_void, CString};
use std::ptr::{addr_of_mut, null_mut};
#[repr(C)]
#[derive(Debug)]
enum TaskState {
  #[doc = "When a task is created or loaded, either explicitly or implicitly, it is in the Unverified
state. In this state, you configure the timing, triggering, and channel attributes
properties of the task."]
  Unverified,
  Verified,
  Reserved,
  Committed,
  Running,
}

#[repr(C)]
#[derive(Debug)]
pub struct Task {
  name: CString,
  handle: TaskHandle,
  channels: HashMap<String, VirtualChannel>,
  state: TaskState,
}

#[repr(C)]
#[derive(Debug)]
struct TaskHandle(*mut c_void);

#[repr(C)]
#[derive(Debug)]
pub struct TaskController {
  parent: Task,
}

impl TaskController {
  fn new(task: Task) -> TaskController {
    TaskController { parent: task }
  }
  pub fn start(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Start)
    })?;
    Ok(())
  }
  pub fn stop(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Stop)
    })?;
    Ok(())
  }
  pub fn verify(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Verify)
    })?;
    Ok(())
  }
  pub fn commit(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Commit)
    })?;
    Ok(())
  }
  pub fn reserve(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Reserve)
    })?;
    Ok(())
  }
  pub fn unreserve(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Unreserve)
    })?;
    Ok(())
  }
  pub fn abort(&mut self) -> Result<()> {
    daqmx_check(unsafe {
      DAQmxTaskControl(self.parent.handle.0, DAQmx_Val_Task_Abort)
    })?;
    Ok(())
  }
}

impl Task {
  pub fn new(name: &str) -> Result<Task> {
    let mut task = Task {
      name: CString::new(name).expect("Task name cannot include 0"),
      handle: TaskHandle(std::ptr::null_mut()),
      channels: HashMap::new(),
      state: TaskState::Unverified,
    };
    let ret = unsafe {
      DAQmxCreateTask(task.name.as_ptr(), addr_of_mut!(task.handle.0))
    };
    daqmx_check(ret)?;
    Ok(task)
  }
  pub fn create(name: &str) -> Result<Task> {
    Task::new(name)
  }
  pub fn start(&mut self) -> Result<&Self> {
    daqmx_check(unsafe { DAQmxStartTask(self.handle.0) })?;
    Ok(self)
  }
  pub fn clear(&mut self) -> Result<&Self> {
    daqmx_check(unsafe { DAQmxClearTask(self.handle.0) })?;
    Ok(self)
  }
  pub fn stop(&mut self) -> Result<&Self> {
    daqmx_check(unsafe { DAQmxStopTask(self.handle.0) })?;
    Ok(self)
  }
  pub fn load(task_name: &str) -> Result<Task> {
    let name = CString::new(task_name).expect("Task name cannot include 0");
    let mut handle = TaskHandle(null_mut());
    daqmx_check(unsafe {
      DAQmxLoadTask(name.as_ptr(), addr_of_mut!(handle.0))
    })?;
    let task = Task {
      name,
      handle,
      channels: HashMap::new(),
      state: TaskState::Unverified,
    };
    Ok(task)
  }
  // pub fn is_done(&mut self) -> Result<()> {
  //     daqmx_check(unsafe { DAQmxIsTaskDone(self.handle.0) })?;
  // }
  pub fn wait_until_done(&mut self) -> Result<()> {
    daqmx_check(unsafe { DAQmxStopTask(self.handle.0) })?;
    Ok(())
  }
  // pub fn wait_for_valid_timestamp(&mut self) -> Result<()> {
  //     daqmx_check(unsafe { DAQmxWaitForValidTimestamp(self.handle.0) })?;
  //     Ok(())
  // }
  // pub fn add_globchans_to_task(&mut self) -> Result<()> {
  //     daqmx_check(unsafe { DAQmxStopTask(self.handle.0) })?;
  //     Ok(())
  // }

  pub fn take(mut self) -> Result<TaskController> {
    Ok(TaskController::new(self))
  }
}

#[test]
fn test_create_task() {
  let task = Task::new("task1");
  println!("Task: {:#?}", task)
}

#[test]
fn test_start_task() {
  let mut task = Task::new("task_start").unwrap();
  task.start();
  println!("Task started: {:#?}", task)
}

#[test]
fn test_stop_task() {
  let mut task = Task::new("task_stop").unwrap();
  task.start();
  println!("Task started: {:#?}", task);
  // task.stop();
  // println!("Task stopped: {:#?}", task)
}
#[test]
fn test_load_task() {
  let loaded_task = Task::load("DummyTaskAICurrent").unwrap();
  println!("Task loaded: {:#?}", loaded_task);
  test_clear_task(loaded_task);
}

#[cfg(test)]
fn test_clear_task(mut task: Task) {
  task.clear().unwrap();
  println!("Task cleared: {:#?}", &task);
}
// int32 __CFUNC     namear taskName[], TaskHandle *taskHandle);
// int32 __CFUNC     DAQmxCreateTask                (const char taskName[], TaskHandle *taskHandle);
// // Channel Names must be valid channels already available in MAX. They are not created.
// int32 __CFUNC     DAQmxAddGlobalChansToTask      (TaskHandle taskHandle, const char channelNames[]);

// int32 __CFUNC     DAQmxStartTask                 (TaskHandle taskHandle);
// int32 __CFUNC     DAQmxStopTask                  (TaskHandle taskHandle);

// namet32 __CFUNC     DAQmxClearTask                 (TaskHandle taskHandlnamet32 __CFUNC     DAQmxWaitUntilTaskDone         (TaskHandle taskHandle, float64 timeToWait);
// t32 __CFUNC     DAQmxWaitForValidTimestamp     (TaskHandle taskHandle, int32 Task, float64 timeout, CVIAbsoluteTime* timestamp);
// Taskint32 __CFUNC     DAQmxIsTaskDone                (TaskHandle taskHandle, bool32 *isTaskDone);

// int32 __CFUNC     DAQmxTaskControl               (TaskHandle taskHandle, int32 action);
// int32 __CFUNC     DAQmxGetNthTaskChannel         (TaskHandle taskHandle, uInt32 index, char buffer[], int32 bufferSize);

// int32 __CFUNC     DAQmxGetNthTaskDevice          (TaskHandle taskHandle, uInt32 index, char buffer[], int32 bufferSize);

// int32 __CFUNC_C   DAQmxGetTaskAttribute          (TaskHandle taskHandle, int32 attribute, void *value, ...);
