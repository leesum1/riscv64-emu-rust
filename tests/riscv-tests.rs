extern crate rv64emu;
use std::{fs, path::Path, rc::Rc};

use log::LevelFilter;
use rv64emu::{
    config::Config,
    device::device_memory::DeviceMemory,
    rvsim::RVsim,
    tools::{rc_refcell_new, RcRefCell},
};

use crate::{
    rv64emu::device::device_trait::{DeviceBase, MEM_BASE},
    rv64emu::rv64core::bus::{Bus, DeviceType},
    rv64emu::rv64core::cpu_core::CpuCoreBuild,
};

fn get_riscv_tests_path() -> std::path::PathBuf {
    let root_dir: &str = env!("CARGO_MANIFEST_DIR");
    let elf_path: std::path::PathBuf = Path::new(root_dir)
        .join("ready_to_run")
        .join("riscv-tests")
        .join("elf");
    elf_path
}

// ture: pass, false: fail
fn start_test(img: &str) -> bool {
    // let bus_u = Rc::new(Mutex::new(Bus::new()));
    let bus_u: RcRefCell<Bus> = RcRefCell::new(Bus::new().into());

    let mut config = Config::new();
    config.set_tlb_size(256);
    config.set_icache_size(4096);
    config.set_decode_cache_size(4096);
    config.set_isa("rv64ima");
    config.set_mmu_type("sv39");
    config.set_s_mode();

    let config = Rc::new(config);

    let cpu = rc_refcell_new(
        CpuCoreBuild::new(bus_u.clone(), config)
            .with_boot_pc(0x8000_0000)
            .with_hart_id(0)
            .with_smode(true)
            .build(),
    );

    // device dram
    let mem: DeviceMemory = DeviceMemory::new(128 * 1024 * 1024);
    let device_name = mem.get_name();
    bus_u.borrow_mut().add_device(DeviceType {
        start: MEM_BASE,
        len: mem.size() as u64,
        instance: Box::new(mem),
        name: device_name,
    });

    let mut sim = RVsim::new(vec![cpu], 23456);

    sim.load_image(img);

    sim.run()
}

#[test]
fn test_once() {
    let img = get_riscv_tests_path().join("rv64mi-p-csr");
    let ret = start_test(img.to_str().unwrap());
    assert!(ret);
}

struct TestRet {
    pub name: String,
    pub ret: bool,
}

#[test]
fn run_arch_tests() {
    // not support misaligned load/store, so skip these tests

    let sikp_files = [
        "rv64ui-p-ma_data",
        "rv64ui-v-ma_data",
        "rv64mi-p-breakpoint",
    ];
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();

    let tests_dir = get_riscv_tests_path();
    let mut tests_ret: Vec<TestRet> = Vec::new();

    for entry in fs::read_dir(tests_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();

        let file_name = path.file_name().unwrap().to_str().unwrap();
        if sikp_files.contains(&file_name) {
            continue;
        }
        if let Some(p) = path.to_str() {
            let ret = start_test(p);
            tests_ret.push(TestRet {
                name: String::from(file_name),
                ret,
            });
        }
    }

    tests_ret
        .iter()
        .filter(|item| item.ret)
        .for_each(|x| println!("{:40}{}", x.name, x.ret));
    tests_ret
        .iter()
        .filter(|item| !item.ret)
        .for_each(|x| println!("{:40}{}", x.name, x.ret));

    tests_ret.iter().for_each(|x| {
        assert!(x.ret);
    });
}
