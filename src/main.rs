//#[test]
//fn should_fail() {
//  unimplemented!();
//}

// pub unsafe extern "system" fn SetupDiGetClassDevsW(
//   ClassGuid: *const GUID,
//   Enumerator: PCWSTR,
//   hwndParent: HWND,
//   Flags: DWORD
//) -> HDEVINFO

// pub unsafe extern "system" fn GetLastError() -> DWORD

// pub unsafe extern "system" fn SetupDiDestroyDeviceInfoList(
//   DevInfoSet: HDEVINFO
// ) -> BOOL

#[cfg(test0)]
fn do_stuff() -> bool {
  use ferris_says::say;
  use std::io::{stdout, BufWriter};

  let stdout = stdout();

  let message = String::from("Hello fellow Rustaceans!");
  let width = message.chars().count();

  let mut writer = BufWriter::new(stdout.lock());
  say(message.as_bytes(), width, &mut writer).unwrap();

  return true;
}

#[cfg(test1)]
fn do_stuff() -> bool {
  println!("Function call!");
  return true;
}

#[cfg(test2)]
fn do_stuff() -> bool {

  use winapi::um::{
    winuser::{MB_OK, MB_ICONINFORMATION, MessageBoxW},
    errhandlingapi::{GetLastError}
  };

  println!("Function call!");
  let vlasterror;

  let lp_text: Vec<u16> = "Hello, World \u{1F60E}\0".encode_utf16().collect();
  let lp_caption: Vec<u16> = "MessageBox Example\0".encode_utf16().collect();

  unsafe {
     MessageBoxW(
        std::ptr::null_mut(),
        lp_text.as_ptr(),
        lp_caption.as_ptr(),
        MB_OK | MB_ICONINFORMATION 
     );
     vlasterror = GetLastError();
  }

  println!("GetLastError is {}", vlasterror);

  return true;
}


// Some stuff we need...
use winapi::shared::minwindef::{
  TRUE,
  FALSE,
  BOOL,
  DWORD
};

use winapi::shared::winerror::{
  // ERROR_SUCCESS,
  ERROR_NO_MORE_ITEMS,
  //ERROR_INSUFFICIENT_BUFFER
};

use std::mem::{
  size_of
};

use winapi::um::{
  handleapi::{INVALID_HANDLE_VALUE},
  errhandlingapi::{GetLastError},
  setupapi::{
      HDEVINFO,
      SP_DEVINFO_DATA,
      PSP_DEVINFO_DATA,
      DIGCF_DEVICEINTERFACE, DIGCF_PRESENT,
      SetupDiGetClassDevsW,
      SetupDiEnumDeviceInfo,
      SetupDiGetDeviceInstanceIdW,
      //SetupDiGetDeviceRegistryPropertyW,
      SetupDiDestroyDeviceInfoList
    },
    cfgmgr32::{
      CR_SUCCESS,
      // CONFIGRET,
      DEVINST,
      CM_Get_Parent,
      CM_Get_Device_IDW
    }
};

// Some handy GUID stuff
use winapi::shared::guiddef::GUID;

// GUID_DEVINTERFACE_USB_DEVICE
// {A5DCBF10-6530-11D2-901F-00C04FB951ED}
#[cfg(unused)]
const GUID_DEVINTERFACE_USB_DEVICE:GUID = GUID {
         Data1:0xA5DCBF10,
         Data2:0x6530,
         Data3:0x11D2,
         Data4:[0x90, 0x1F, 0x00, 0xC0, 0x4F, 0xB9, 0x51, 0xED] 
      };

// GUID_DEVINTERFACE_HID
// {4D1E55B2-F16F-11CF-88CB-001111000030}
const GUID_DEVINTERFACE_HID:GUID = GUID {
         Data1:0x4D1E55B2,
         Data2:0xF16F,
         Data3:0x11CF,
         Data4:[0x88, 0xCB, 0x00, 0x11, 0x11, 0x00, 0x00, 0x30] 
      };

// Appears we need to declare device registry entry property numbers
//const SDRP_HARDWAREID:DWORD = 0x00000000;

fn has_usb_parent(h_dev_inst:DEVINST) -> bool {
  let mut b_rval:bool = false;

  let mut dev_curr:DEVINST = h_dev_inst;
  let mut dev_parent:DEVINST = 0;

  // hard coding buffer strings here until I find a better way to do this
  let mut buffer: Vec<u16> = Vec::with_capacity(255);

  loop
  {
    unsafe {
      buffer.set_len(255);
      if CM_Get_Device_IDW(dev_curr, buffer.as_mut_ptr(), 255, 0) != CR_SUCCESS {
        break;
      };
      let str_result = String::from_utf16_lossy(&buffer[0..254]);

      //println!("Climbing the class ladder with strings {}", str_result);
      if str_result.contains("USB") {
        //println!("Climbing class ladder usb found {}", str_result);
        b_rval = true;
        break;
      }

      if CM_Get_Parent(&mut dev_parent, dev_curr, 0) != CR_SUCCESS {
        break;
      };
      dev_curr = dev_parent;
    }; 
  };

  return b_rval;
}

#[cfg(unused)]
fn get_print_device_property (
  hdev_info:HDEVINFO,
  pdev_info_data:PSP_DEVINFO_DATA,
  property:DWORD,
  property_name:String
) {

  let b_valid:BOOL;
  let last_error:DWORD;
  let mut required_length;
  let str_result;

  // BUGBUG
  // hard coding buffer strings here until I find a better way to do this
  let mut buffer: Vec<u16> = Vec::with_capacity(255);

  unsafe {
    buffer.set_len(255);
    
    b_valid = SetupDiGetDeviceRegistryPropertyW(
      hdev_info,
      pdev_info_data,
      property,
      std::ptr::null_mut(),
      buffer.as_mut_ptr(),
      255,
      &mut required_length
    );
    last_error = GetLastError();

    if b_valid == TRUE {
      str_result = String::from_utf16_lossy(&buffer[0..required_length as usize]);
    }
    else {
      str_result = format!("ERROR {}", last_error);
    }
  }

  println!("      property {} {{ {}", property_name, str_result);
 
}

fn print_dev_info(hdev_info:HDEVINFO, pdev_info_data:PSP_DEVINFO_DATA) {
  println!("  HidDevice {{");

  let b_valid:BOOL;
  let lasterror:DWORD;

  // BUGBUG
  // hard coding buffer strings here until I find a better way to do this
  let mut buffer: Vec<u16> = Vec::with_capacity(255);

  let mut n_size:DWORD = 0;
  let str_result;

  // Get device specific stuff, device ID
  unsafe {
  
    buffer.set_len(255);
       
    b_valid = SetupDiGetDeviceInstanceIdW(hdev_info, pdev_info_data, buffer.as_mut_ptr(), buffer.len() as DWORD, &mut n_size);
    lasterror = GetLastError();

    if b_valid == TRUE {
      str_result = String::from_utf16_lossy(&buffer[0..n_size as usize]);
    }
    else {
      str_result = format!("ERROR {}", lasterror);
    }
  };
  println!("    device identifier: {}", str_result);

  println!("  }}");

}

fn do_stuff() -> bool {

  let hdev_info_set:HDEVINFO;
  let mut b_valid:BOOL;
  let mut lasterror:DWORD;

  unsafe {
    hdev_info_set = SetupDiGetClassDevsW(
      &GUID_DEVINTERFACE_HID,
      std::ptr::null_mut(),
      std::ptr::null_mut(),
      DIGCF_DEVICEINTERFACE | DIGCF_PRESENT
    );
    lasterror = GetLastError();
  }

  if lasterror != 0 {
    println!("Got invalid device interface class error {}", lasterror);
    return false;
  }
  //println!("Got valid device interface HID classes");

  let mut icurrdev = 0;

  loop {
    // let mut device_info_data:SP_DEVINFO_DATA = mem::uninitialized();
    // device_info_data.cbSize = sizeof::<SP_DEEV_INFO_DATA>() as u32;

    let mut device_info_data = SP_DEVINFO_DATA{
       cbSize:size_of::<SP_DEVINFO_DATA>() as u32,
       ClassGuid:GUID{Data1:0,Data2:0,Data3:0,Data4:[0,0,0,0,0,0,0,0]},
       DevInst:0,
       Reserved:0
    };

    unsafe {
      b_valid = SetupDiEnumDeviceInfo(hdev_info_set, icurrdev, &mut device_info_data);
      lasterror = GetLastError();
    };

    if b_valid == FALSE {
      if lasterror != ERROR_NO_MORE_ITEMS {
        println!("SetupDiEnumDeviceInfo error {}", lasterror);
      }
      break;
    }

    if cfg!(do_usb_parent) {
      // Valid device_info_data so let's check for a USB parent
      if !has_usb_parent(device_info_data.DevInst) {
        // No USB parent
        icurrdev += 1;
        continue;
      }

      // valid device info
      // parent is USB device
      // display and get information as appropriate.

      println!("hid devices with USB parent: {{");
    }
    else
    {
      // valid device info
      // display and get information as appropriate.

      println!("hid devices: {{");
    }

    // print the single device info stuff
    print_dev_info(hdev_info_set, &mut device_info_data);

    // print device specific information

    // product_id is needs some string parsing...
    //get_print_device_property(
    //  hdev_info_set,
    //  &mut device_info_data,
    //  SDRP_HARDWAREID,
    //  String::from("product_id: ")
    //);

    // vendor_id needs some string parsing...
    // another comment to get git to change
    //get_print_device_property(
    //  hdev_info_set,
    //  &mut device_info_data,
    //  SDRP_HARDWAREID,
    //  String::from("vendor_id: ")
    //);

    // TODO
    // go through and print each interface here...

    // Now go to the next device
    icurrdev += 1;

  }

  unsafe {
    if hdev_info_set != INVALID_HANDLE_VALUE {
      SetupDiDestroyDeviceInfoList(hdev_info_set);
      //hdev_info_set = INVALID_HANDLE_VALUE;
    }
  }

  return true;
}

fn main() {
    //println!("Hello, world!");
    do_stuff();
}
