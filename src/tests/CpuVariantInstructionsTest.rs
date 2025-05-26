


#[cfg(test)]
mod CpuVariantInstructionsTest {        
    use crate::core::address::Address;
    use crate::core::registers::{Register,Registers};
    use crate::core::memory::Memory;
    use crate::core::cpuDependencies::CpuDependencies;
    use crate::core::CpuVariantInstructions::CosmacCPU;
    use crate::core::CpuVariantInstructions::CPUVariantInstructionsTrait;
/* 
    fn get_cosmac_cpu(cpuDependencies:&mut CpuDependencies) -> Box<dyn CPUVariantInstructionsTrait> {
        Box::new(
            CosmacCPU::init(&mut cpuDependencies)
        )
    }
    */

    #[test]
    fn opcode_8XY6() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let mut cpu:Box<dyn CPUVariantInstructionsTrait> =        Box::new(
            CosmacCPU::init(&mut cpuDependencies)
        );

            /*
                        CosmacCPU::init(cpuDependencies.get_index(),
                 cpuDependencies.get_pc(),
                 cpuDependencies.get_stack(), 
                 cpuDependencies.get_registers(), 
                 cpuDependencies.get_memory(),  
                 cpuDependencies.get_display(),
                 cpuDependencies.get_keyboard(),
                 cpuDependencies.get_timers(), 
                 &mut cpuDependencies)
            
             */
        let instruction:u16 = 0x8346;

        (*cpu).opcode_8XY6(instruction);
        let vx:&mut Register = (*cpu).get_x_register(instruction);
        let vx_val:u8 = vx.get();

        let vy:&mut Register = (*cpu).get_y_register(instruction);
        let vy_val:u8 = vy.get();

        assert_eq!(vx_val,vy_val >> 1);
        // test carry flag
        
        assert_eq!((*cpu).get_x_register(0x0F00).get() == 1, vy_val & 0x1 == 1);
        
    }

    #[test]
    fn opcode_8XYE() {
        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let mut cpu:Box<dyn CPUVariantInstructionsTrait> =        Box::new(
            CosmacCPU::init(&mut cpuDependencies)
        );
        let instruction:u16 = 0x834E;

        cpu.opcode_8XYE(instruction);

        let vx:&mut Register = cpu.get_x_register(instruction);
        let vx_val:u8 = vx.get();

        let vy:&mut Register = cpu.get_y_register(instruction);
        let vy_val:u8 = vy.get();

        let result:u8 = vy_val << 1;

    
        assert_eq!(vx_val,result);
        // test carry flag
        assert_eq!((*cpu).get_x_register(0x0F00).get() == 1, (vy_val & 0x80) > 0);

    }


    #[test]
    fn opcode_BXNN() {
        let mut cpuDependencies:Box<CpuDependencies> = Box::new(CpuDependencies::init());
        let pc_val:u16 = (*cpuDependencies).pc.get();

        let v0_val = (*cpuDependencies).registers.get(0).get();

        let mut cpu:Box<dyn CPUVariantInstructionsTrait> =  Box::new(
            CosmacCPU::init(&mut *cpuDependencies)
        );

        let instruction:u16 = 0xB346;

        let nnn:u16 = (*cpu).get_nnn(instruction);
        (*cpu).opcode_BXNN(instruction);
        drop(cpu);
        let pc = (*cpuDependencies).pc.get();
  
        assert_eq!(pc,(v0_val as u16) + nnn);


    }

    #[test]
    fn opcode_FX55() {

        let mut cpuDependencies:CpuDependencies = CpuDependencies::init();
        let mut index:&mut Address = cpuDependencies.get_index();
        index.set(0x200);

        let mut cpu:Box<dyn CPUVariantInstructionsTrait> =        Box::new(
            CosmacCPU::init(&mut cpuDependencies)
        );
        let instruction:u16 = 0xF355;

        let vx:&mut Register =  (*cpu).get_x_register(instruction);
        vx.set(12);
        let mut val:u8 = vx.get();
        
        (*cpu).opcode_FX55(instruction);
        drop(cpu);

        for i in 0..val {
            let byte:u8 = cpuDependencies.get_memory().read((0x200 as u16 +i as u16) as u16);
            assert_eq!(byte,cpuDependencies.registers.get(i).get());
        }

    }

    #[test]
    fn opcode_FX65() {
        let instruction:u16 = 0xF365;
        let mut cpuDependencies:Box<CpuDependencies> = Box::new(CpuDependencies::init());

        let mut index:&mut Address = (*cpuDependencies).get_index();
        index.set(0x200);



        let mut cpu:Box<dyn CPUVariantInstructionsTrait> =        Box::new(
            CosmacCPU::init(&mut cpuDependencies)
        );

        let vx:&mut Register = (*cpu).get_x_register(instruction);
        vx.set(12);

        let mut val:u8 = vx.get();
        (*cpu).opcode_FX65(instruction);

        drop(cpu);

        for i in 0..val {
            let byte:u8  = cpuDependencies.get_memory().read(0x200 as u16 + i as u16);
            assert_eq!(byte, cpuDependencies.registers.get(i as u8).get());
        }

    }
    
}