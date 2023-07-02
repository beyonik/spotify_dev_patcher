use toy_arms::external::process::Process;
use toy_arms::external::{read, write};

const I386_DEV_CHECK: &str = "81 7D 08 18 F6 00 00";
const X64_DEV_CHECK: &str = "81 FA 18 F6 00 00";

fn main() {
    let spotify = match Process::from_process_name("Spotify.exe") {
        Ok(proc) => proc,
        Err(e) => {
            println!("[!] failed to find spotify.exe ({:#?})", e);
            std::io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    };

    println!("[*] found spotify with procid: {}", spotify.id);

    let mut spotify_base_module = match spotify.get_module_info("Spotify.exe") {
        Ok(module) => module,
        Err(e) => {
            println!("[!] failed to find spotify base module ({:#?})", e);
            std::io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    };

    println!("[*] spotify base: {:#x}", spotify_base_module.base_address);

    let is_32bit = !(spotify_base_module.base_address > 0xFFFFFFFF);

    let dev_check_address_offset = match is_32bit {
        true => match spotify_base_module.find_pattern(I386_DEV_CHECK) {
            Some(addr) => addr,
            None => {
                println!("[!] failed to find i386 pattern for dev check!");
                std::io::stdin().read_line(&mut String::new()).unwrap();
                return;
            }
        }
        false => match spotify_base_module.find_pattern(X64_DEV_CHECK) {
            Some(addr) => addr,
            None => {
                println!("[!] failed to find x64 pattern for dev check!");
                std::io::stdin().read_line(&mut String::new()).unwrap();
                return;
            }
        }
    };

    let dev_check_address = spotify_base_module.base_address + dev_check_address_offset;

    println!("[*] dev check: {:#x}", dev_check_address);

    let mut dev_mode_check_call = dev_check_address + 0x9;

    if is_32bit {
        dev_mode_check_call += 0x1;
    }

    let mut dev_mode_call_offset: u16 = 0;
    match read::<u16>(&spotify.handle, dev_mode_check_call, 0x2, &mut dev_mode_call_offset as *mut u16) {
        Ok(_) => {
            println!("[*] dev mode function offset: {:#x}", dev_mode_call_offset);
        }
        Err(e) => {
            println!("[!] offset read error: {:#?}", e);
            std::io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    }

    let dev_mode_check_func_addr = dev_mode_check_call + dev_mode_call_offset as usize + 0x4;

    println!("[*] dev mode check function: {:#x}", dev_mode_check_func_addr);

    let mut byte_flag_address: usize = 0;

    match read::<usize>(&spotify.handle, dev_mode_check_func_addr + (if is_32bit { 0x1 } else { 0x2 }), 0x4, &mut byte_flag_address as *mut usize) {
        Ok(a) => a,
        Err(e) => {
            println!("[!] dev mode byte flag couldn't be retrieved! {:#?}", e);
            std::io::stdin().read_line(&mut String::new()).unwrap();
            return;
        }
    };

    if !is_32bit {
        byte_flag_address += dev_mode_check_func_addr + 0x6;
    }

    println!("[*] dev mode byte flag: {:#x}", byte_flag_address);

    write::<u8>(&spotify.handle, byte_flag_address, &mut 1).expect("[*] failed to write dev mode byte!");

    println!("[*] dev mode should be enabled");

    std::io::stdin().read_line(&mut String::new()).unwrap();
}
