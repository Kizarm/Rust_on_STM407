use vc::{RWReg,ROReg};

#[repr(C, packed)]
pub struct NVIC_Type {
  pub iser       :[RWReg<u32>; 8],    /* Offset: 0x000 (R/W)  Interrupt Set Enable Register           */
      reserved0  :[u32; 24],
  pub icer       :[RWReg<u32>; 8],    /* Offset: 0x080 (R/W)  Interrupt Clear Enable Register         */
  pub rserved1   :[u32; 24],
  pub ispr       :[RWReg<u32>; 8],    /* Offset: 0x100 (R/W)  Interrupt Set Pending Register          */
      reserved2  :[u32; 24],
  pub icpr       :[RWReg<u32>; 8],    /* Offset: 0x180 (R/W)  Interrupt Clear Pending Register        */
      reserved3  :[u32; 24],
  pub iabr       :[RWReg<u32>; 8],    /* Offset: 0x200 (R/W)  Interrupt Active bit Register           */
      reserved4  :[u32; 56],
  pub ip         :[RWReg<u8>; 240],   /* Offset: 0x300 (R/W)  Interrupt Priority Register (8Bit wide) */
      reserved5  :[u32; 644],
  pub stir       : RWReg<u32>,        /* Offset: 0xE00 ( /W)  Software Trigger Interrupt Register     */
}/* 0x0E04 */

#[repr(C, packed)]
pub struct SCB_Type {
  pub cpuid      : ROReg<u32>,        /* Offset: 0x000 (R/ )  CPUID Base Register                                   */
  pub icsr       : RWReg<u32>,        /* Offset: 0x004 (R/W)  Interrupt Control and State Register                  */
  pub vtor       : RWReg<u32>,        /* Offset: 0x008 (R/W)  Vector Table Offset Register                          */
  pub aircr      : RWReg<u32>,        /* Offset: 0x00C (R/W)  Application Interrupt and Reset Control Register      */
  pub scr        : RWReg<u32>,        /* Offset: 0x010 (R/W)  System Control Register                               */
  pub ccr        : RWReg<u32>,        /* Offset: 0x014 (R/W)  Configuration Control Register                        */
  pub shp        :[RWReg<u8>; 12],    /* Offset: 0x018 (R/W)  System Handlers Priority Registers (4-7, 8-11, 12-15) */
  pub shcsr      : RWReg<u32>,        /* Offset: 0x024 (R/W)  System Handler Control and State Register             */
  pub cfsr       : RWReg<u32>,        /* Offset: 0x028 (R/W)  Configurable Fault Status Register                    */
  pub hfsr       : RWReg<u32>,        /* Offset: 0x02C (R/W)  HardFault Status Register                             */
  pub dfsr       : RWReg<u32>,        /* Offset: 0x030 (R/W)  Debug Fault Status Register                           */
  pub mmfar      : RWReg<u32>,        /* Offset: 0x034 (R/W)  MemManage Fault Address Register                      */
  pub bfar       : RWReg<u32>,        /* Offset: 0x038 (R/W)  BusFault Address Register                             */
  pub afsr       : RWReg<u32>,        /* Offset: 0x03C (R/W)  Auxiliary Fault Status Register                       */
  pub pfr        :[ROReg<u32>; 2],    /* Offset: 0x040 (R/ )  Processor Feature Register                            */
  pub dfr        : ROReg<u32>,        /* Offset: 0x048 (R/ )  Debug Feature Register                                */
  pub adr        : ROReg<u32>,        /* Offset: 0x04C (R/ )  Auxiliary Feature Register                            */
  pub mmfr       :[ROReg<u32>; 4],    /* Offset: 0x050 (R/ )  Memory Model Feature Register                         */
  pub isar       :[ROReg<u32>; 5],    /* Offset: 0x060 (R/ )  Instruction Set Attributes Register                   */
      reserved0  :[u32; 5],
  pub cpacr      : RWReg<u32>,        /* Offset: 0x088 (R/W)  Coprocessor Access Control Register                   */
}/* 0x008C */

#[repr(C, packed)]
pub struct SCnSCB_Type {
      reserved0  :[u32; 1],
  pub ictr       : ROReg<u32>,        /* Offset: 0x004 (R/ )  Interrupt Controller Type Register      */
  pub actlr      : RWReg<u32>,        /* Offset: 0x008 (R/W)  Auxiliary Control Register              */
}/* 0x000C */

#[repr(C, packed)]
pub struct SysTick_Type {
  pub ctrl       : RWReg<u32>,        /* Offset: 0x000 (R/W)  SysTick Control and Status Register */
  pub load       : RWReg<u32>,        /* Offset: 0x004 (R/W)  SysTick Reload Value Register       */
  pub val        : RWReg<u32>,        /* Offset: 0x008 (R/W)  SysTick Current Value Register      */
  pub calib      : ROReg<u32>,        /* Offset: 0x00C (R/ )  SysTick Calibration Register        */
}/* 0x0010 */

#[repr(C, packed)]
pub struct ITM_Type {
  pub port       :[RWReg<u32>; 32],   /* Offset: 0x000 ( /W)  ITM Stimulus Port 32-bit                  */
      reserved0  :[u32; 864],
  pub ter        : RWReg<u32>,        /* Offset: 0xE00 (R/W)  ITM Trace Enable Register                 */
      reserved1  :[u32; 15],
  pub tpr        : RWReg<u32>,        /* Offset: 0xE40 (R/W)  ITM Trace Privilege Register              */
      reserved2  :[u32; 15],
  pub tcr        : RWReg<u32>,        /* Offset: 0xE80 (R/W)  ITM Trace Control Register                */
}/* 0x0E84 */

#[repr(C, packed)]
pub struct CoreDebug_Type {
  pub dhcsr      : RWReg<u32>,        /* Offset: 0x000 (R/W)  Debug Halting Control and Status Register    */
  pub dcrsr      : RWReg<u32>,        /* Offset: 0x004 ( /W)  Debug Core Register Selector Register        */
  pub dcrdr      : RWReg<u32>,        /* Offset: 0x008 (R/W)  Debug Core Register Data Register            */
  pub demcr      : RWReg<u32>,        /* Offset: 0x00C (R/W)  Debug Exception and Monitor Control Register */
}/* 0x0010 */

#[repr(C, packed)]
pub struct ADC_Type {
  pub sr         : RWReg<u32>,        /* ADC status register,                         Address offset: 0x00 */
  pub cr1        : RWReg<u32>,        /* ADC control register 1,                      Address offset: 0x04 */
  pub cr2        : RWReg<u32>,        /* ADC control register 2,                      Address offset: 0x08 */
  pub smpr1      : RWReg<u32>,        /* ADC sample time register 1,                  Address offset: 0x0C */
  pub smpr2      : RWReg<u32>,        /* ADC sample time register 2,                  Address offset: 0x10 */
  pub jofr1      : RWReg<u32>,        /* ADC injected channel data offset register 1, Address offset: 0x14 */
  pub jofr2      : RWReg<u32>,        /* ADC injected channel data offset register 2, Address offset: 0x18 */
  pub jofr3      : RWReg<u32>,        /* ADC injected channel data offset register 3, Address offset: 0x1C */
  pub jofr4      : RWReg<u32>,        /* ADC injected channel data offset register 4, Address offset: 0x20 */
  pub htr        : RWReg<u32>,        /* ADC watchdog higher threshold register,      Address offset: 0x24 */
  pub ltr        : RWReg<u32>,        /* ADC watchdog lower threshold register,       Address offset: 0x28 */
  pub sqr1       : RWReg<u32>,        /* ADC regular sequence register 1,             Address offset: 0x2C */
  pub sqr2       : RWReg<u32>,        /* ADC regular sequence register 2,             Address offset: 0x30 */
  pub sqr3       : RWReg<u32>,        /* ADC regular sequence register 3,             Address offset: 0x34 */
  pub jsqr       : RWReg<u32>,        /* ADC injected sequence register,              Address offset: 0x38*/
  pub jdr1       : RWReg<u32>,        /* ADC injected data register 1,                Address offset: 0x3C */
  pub jdr2       : RWReg<u32>,        /* ADC injected data register 2,                Address offset: 0x40 */
  pub jdr3       : RWReg<u32>,        /* ADC injected data register 3,                Address offset: 0x44 */
  pub jdr4       : RWReg<u32>,        /* ADC injected data register 4,                Address offset: 0x48 */
  pub dr         : RWReg<u32>,        /* ADC regular data register,                   Address offset: 0x4C */
}/* 0x0050 */

#[repr(C, packed)]
pub struct ADC_Common_Type {
  pub csr        : RWReg<u32>,        /* ADC Common status register,                  Address offset: ADC1 base address + 0x300 */
  pub ccr        : RWReg<u32>,        /* ADC common control register,                 Address offset: ADC1 base address + 0x304 */
  pub cdr        : RWReg<u32>,        /* ADC common regular data register for dual                             AND triple modes,                            Address offset: ADC1 base address + 0x308 */
}/* 0x000C */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CAN_TxMailBox_Type {
  pub tir        : RWReg<u32>,        /* CAN TX mailbox identifier register */
  pub tdtr       : RWReg<u32>,        /* CAN mailbox data length control and time stamp register */
  pub tdlr       : RWReg<u32>,        /* CAN mailbox data low register */
  pub tdhr       : RWReg<u32>,        /* CAN mailbox data high register */
}/* 0x0010 */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CAN_FIFOMailBox_Type {
  pub rir        : RWReg<u32>,        /* CAN receive FIFO mailbox identifier register */
  pub rdtr       : RWReg<u32>,        /* CAN receive FIFO mailbox data length control and time stamp register */
  pub rdlr       : RWReg<u32>,        /* CAN receive FIFO mailbox data low register */
  pub rdhr       : RWReg<u32>,        /* CAN receive FIFO mailbox data high register */
}/* 0x0010 */

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct CAN_FilterRegister_Type {
  pub fr1        : RWReg<u32>,        /* CAN Filter bank register 1 */
  pub fr2        : RWReg<u32>,        /* CAN Filter bank register 1 */
}/* 0x0008 */

#[repr(C, packed)]
pub struct CAN_Type {
  pub mcr        : RWReg<u32>,        /* CAN master control register,         Address offset: 0x00          */
  pub msr        : RWReg<u32>,        /* CAN master status register,          Address offset: 0x04          */
  pub tsr        : RWReg<u32>,        /* CAN transmit status register,        Address offset: 0x08          */
  pub rf0r       : RWReg<u32>,        /* CAN receive FIFO 0 register,         Address offset: 0x0C          */
  pub rf1r       : RWReg<u32>,        /* CAN receive FIFO 1 register,         Address offset: 0x10          */
  pub ier        : RWReg<u32>,        /* CAN interrupt enable register,       Address offset: 0x14          */
  pub esr        : RWReg<u32>,        /* CAN error status register,           Address offset: 0x18          */
  pub btr        : RWReg<u32>,        /* CAN bit timing register,             Address offset: 0x1C          */
      reserved0  :[u32; 88],          /* Reserved, 0x020 - 0x17F                                            */
  pub stxmailbox :[CAN_TxMailBox_Type; 3],/* CAN Tx MailBox,                      Address offset: 0x180 - 0x1AC */
  pub sfifomailbox :[CAN_FIFOMailBox_Type; 2],/* CAN FIFO MailBox,                    Address offset: 0x1B0 - 0x1CC */
      reserved1  :[u32; 12],          /* Reserved, 0x1D0 - 0x1FF                                            */
  pub fmr        : RWReg<u32>,        /* CAN filter master register,          Address offset: 0x200         */
  pub fm1r       : RWReg<u32>,        /* CAN filter mode register,            Address offset: 0x204         */
      reserved2  : u32,               /* Reserved, 0x208                                                    */
  pub fs1r       : RWReg<u32>,        /* CAN filter scale register,           Address offset: 0x20C         */
      reserved3  : u32,               /* Reserved, 0x210                                                    */
  pub ffa1r      : RWReg<u32>,        /* CAN filter FIFO assignment register, Address offset: 0x214         */
      reserved4  : u32,               /* Reserved, 0x218                                                    */
  pub fa1r       : RWReg<u32>,        /* CAN filter activation register,      Address offset: 0x21C         */
      reserved5  :[u32; 8],           /* Reserved, 0x220-0x23F                                              */
  pub sfilterregister :[CAN_FilterRegister_Type; 28],/* CAN Filter Register,                 Address offset: 0x240-0x31C   */
}/* 0x0320 */

#[repr(C, packed)]
pub struct CRC_Type {
  pub dr         : RWReg<u32>,        /* CRC Data register,             Address offset: 0x00 */
  pub idr        : RWReg<u8>,         /* CRC Independent data register, Address offset: 0x04 */
      reserved0  : u8,                /* Reserved, 0x05                                      */
      reserved1  : u16,               /* Reserved, 0x06                                      */
  pub cr         : RWReg<u32>,        /* CRC Control register,          Address offset: 0x08 */
}/* 0x000C */

#[repr(C, packed)]
pub struct DAC_Type {
  pub cr         : RWReg<u32>,        /* DAC control register,                                    Address offset: 0x00 */
  pub swtrigr    : RWReg<u32>,        /* DAC software trigger register,                           Address offset: 0x04 */
  pub dhr12r1    : RWReg<u32>,        /* DAC channel1 12-bit right-aligned data holding register, Address offset: 0x08 */
  pub dhr12l1    : RWReg<u32>,        /* DAC channel1 12-bit left aligned data holding register,  Address offset: 0x0C */
  pub dhr8r1     : RWReg<u32>,        /* DAC channel1 8-bit right aligned data holding register,  Address offset: 0x10 */
  pub dhr12r2    : RWReg<u32>,        /* DAC channel2 12-bit right aligned data holding register, Address offset: 0x14 */
  pub dhr12l2    : RWReg<u32>,        /* DAC channel2 12-bit left aligned data holding register,  Address offset: 0x18 */
  pub dhr8r2     : RWReg<u32>,        /* DAC channel2 8-bit right-aligned data holding register,  Address offset: 0x1C */
  pub dhr12rd    : RWReg<u32>,        /* Dual DAC 12-bit right-aligned data holding register,     Address offset: 0x20 */
  pub dhr12ld    : RWReg<u32>,        /* DUAL DAC 12-bit left aligned data holding register,      Address offset: 0x24 */
  pub dhr8rd     : RWReg<u32>,        /* DUAL DAC 8-bit right aligned data holding register,      Address offset: 0x28 */
  pub dor1       : RWReg<u32>,        /* DAC channel1 data output register,                       Address offset: 0x2C */
  pub dor2       : RWReg<u32>,        /* DAC channel2 data output register,                       Address offset: 0x30 */
  pub sr         : RWReg<u32>,        /* DAC status register,                                     Address offset: 0x34 */
}/* 0x0038 */

#[repr(C, packed)]
pub struct DBGMCU_Type {
  pub idcode     : RWReg<u32>,        /* MCU device ID code,               Address offset: 0x00 */
  pub cr         : RWReg<u32>,        /* Debug MCU configuration register, Address offset: 0x04 */
  pub apb1fz     : RWReg<u32>,        /* Debug MCU APB1 freeze register,   Address offset: 0x08 */
  pub apb2fz     : RWReg<u32>,        /* Debug MCU APB2 freeze register,   Address offset: 0x0C */
}/* 0x0010 */

#[repr(C, packed)]
pub struct DCMI_Type {
  pub cr         : RWReg<u32>,        /* DCMI control register 1,                       Address offset: 0x00 */
  pub sr         : RWReg<u32>,        /* DCMI status register,                          Address offset: 0x04 */
  pub risr       : RWReg<u32>,        /* DCMI raw interrupt status register,            Address offset: 0x08 */
  pub ier        : RWReg<u32>,        /* DCMI interrupt enable register,                Address offset: 0x0C */
  pub misr       : RWReg<u32>,        /* DCMI masked interrupt status register,         Address offset: 0x10 */
  pub icr        : RWReg<u32>,        /* DCMI interrupt clear register,                 Address offset: 0x14 */
  pub escr       : RWReg<u32>,        /* DCMI embedded synchronization code register,   Address offset: 0x18 */
  pub esur       : RWReg<u32>,        /* DCMI embedded synchronization unmask register, Address offset: 0x1C */
  pub cwstrtr    : RWReg<u32>,        /* DCMI crop window start,                        Address offset: 0x20 */
  pub cwsizer    : RWReg<u32>,        /* DCMI crop window size,                         Address offset: 0x24 */
  pub dr         : RWReg<u32>,        /* DCMI data register,                            Address offset: 0x28 */
}/* 0x002C */

#[repr(C, packed)]
pub struct DMA_Stream_Type {
  pub cr         : RWReg<u32>,        /* DMA stream x configuration register      */
  pub ndtr       : RWReg<u32>,        /* DMA stream x number of data register     */
  pub par        : RWReg<u32>,        /* DMA stream x peripheral address register */
  pub m0ar       : RWReg<u32>,        /* DMA stream x memory 0 address register   */
  pub m1ar       : RWReg<u32>,        /* DMA stream x memory 1 address register   */
  pub fcr        : RWReg<u32>,        /* DMA stream x FIFO control register       */
}/* 0x0018 */

#[repr(C, packed)]
pub struct DMA_Type {
  pub lisr       : RWReg<u32>,        /* DMA low interrupt status register,      Address offset: 0x00 */
  pub hisr       : RWReg<u32>,        /* DMA high interrupt status register,     Address offset: 0x04 */
  pub lifcr      : RWReg<u32>,        /* DMA low interrupt flag clear register,  Address offset: 0x08 */
  pub hifcr      : RWReg<u32>,        /* DMA high interrupt flag clear register, Address offset: 0x0C */
}/* 0x0010 */

#[repr(C, packed)]
pub struct ETH_Type {
  pub maccr      : RWReg<u32>,        /* tady fakt komentare nejsou */
  pub macffr     : RWReg<u32>,
  pub machthr    : RWReg<u32>,
  pub machtlr    : RWReg<u32>,
  pub macmiiar   : RWReg<u32>,
  pub macmiidr   : RWReg<u32>,
  pub macfcr     : RWReg<u32>,
  pub macvlantr  : RWReg<u32>,
      reserved0  :[u32; 2],
  pub macrwuffr  : RWReg<u32>,
  pub macpmtcsr  : RWReg<u32>,
      reserved1  :[u32; 2],
  pub macsr      : RWReg<u32>,
  pub macimr     : RWReg<u32>,
  pub maca0hr    : RWReg<u32>,
  pub maca0lr    : RWReg<u32>,
  pub maca1hr    : RWReg<u32>,
  pub maca1lr    : RWReg<u32>,
  pub maca2hr    : RWReg<u32>,
  pub maca2lr    : RWReg<u32>,
  pub maca3hr    : RWReg<u32>,
  pub maca3lr    : RWReg<u32>,
      reserved2  :[u32; 40],
  pub mmccr      : RWReg<u32>,
  pub mmcrir     : RWReg<u32>,
  pub mmctir     : RWReg<u32>,
  pub mmcrimr    : RWReg<u32>,
  pub mmctimr    : RWReg<u32>,
      reserved3  :[u32; 14],
  pub mmctgfsccr : RWReg<u32>,
  pub mmctgfmsccr : RWReg<u32>,
      reserved4  :[u32; 5],
  pub mmctgfcr   : RWReg<u32>,
      reserved5  :[u32; 10],
  pub mmcrfcecr  : RWReg<u32>,
  pub mmcrfaecr  : RWReg<u32>,
      reserved6  :[u32; 10],
  pub mmcrgufcr  : RWReg<u32>,
      reserved7  :[u32; 334],
  pub ptptscr    : RWReg<u32>,
  pub ptpssir    : RWReg<u32>,
  pub ptptshr    : RWReg<u32>,
  pub ptptslr    : RWReg<u32>,
  pub ptptshur   : RWReg<u32>,
  pub ptptslur   : RWReg<u32>,
  pub ptptsar    : RWReg<u32>,
  pub ptptthr    : RWReg<u32>,
  pub ptpttlr    : RWReg<u32>,
      reserved8  : RWReg<u32>,
  pub ptptssr    : RWReg<u32>,
      reserved9  :[u32; 565],
  pub dmabmr     : RWReg<u32>,
  pub dmatpdr    : RWReg<u32>,
  pub dmarpdr    : RWReg<u32>,
  pub dmardlar   : RWReg<u32>,
  pub dmatdlar   : RWReg<u32>,
  pub dmasr      : RWReg<u32>,
  pub dmaomr     : RWReg<u32>,
  pub dmaier     : RWReg<u32>,
  pub dmamfbocr  : RWReg<u32>,
  pub dmarswtr   : RWReg<u32>,
      reserved10 :[u32; 8],
  pub dmachtdr   : RWReg<u32>,
  pub dmachrdr   : RWReg<u32>,
  pub dmachtbar  : RWReg<u32>,
  pub dmachrbar  : RWReg<u32>,
}/* 0x1058 */

#[repr(C, packed)]
pub struct EXTI_Type {
  pub imr        : RWReg<u32>,        /* EXTI Interrupt mask register,            Address offset: 0x00 */
  pub emr        : RWReg<u32>,        /* EXTI Event mask register,                Address offset: 0x04 */
  pub rtsr       : RWReg<u32>,        /* EXTI Rising trigger selection register,  Address offset: 0x08 */
  pub ftsr       : RWReg<u32>,        /* EXTI Falling trigger selection register, Address offset: 0x0C */
  pub swier      : RWReg<u32>,        /* EXTI Software interrupt event register,  Address offset: 0x10 */
  pub pr         : RWReg<u32>,        /* EXTI Pending register,                   Address offset: 0x14 */
}/* 0x0018 */

#[repr(C, packed)]
pub struct FLASH_Type {
  pub acr        : RWReg<u32>,        /* FLASH access control register, Address offset: 0x00 */
  pub keyr       : RWReg<u32>,        /* FLASH key register,            Address offset: 0x04 */
  pub optkeyr    : RWReg<u32>,        /* FLASH option key register,     Address offset: 0x08 */
  pub sr         : RWReg<u32>,        /* FLASH status register,         Address offset: 0x0C */
  pub cr         : RWReg<u32>,        /* FLASH control register,        Address offset: 0x10 */
  pub optcr      : RWReg<u32>,        /* FLASH option control register, Address offset: 0x14 */
}/* 0x0018 */

#[repr(C, packed)]
pub struct FSMC_Bank1_Type {
  pub btcr       :[RWReg<u32>; 8],    /* NOR/PSRAM chip-select control register(BCR) and chip-select timing register(BTR), Address offset: 0x00-1C */
}/* 0x0020 */

#[repr(C, packed)]
pub struct FSMC_Bank1E_Type {
  pub bwtr       :[RWReg<u32>; 7],    /* NOR/PSRAM write timing registers, Address offset: 0x104-0x11C */
}/* 0x001C */

#[repr(C, packed)]
pub struct FSMC_Bank2_Type {
  pub pcr2       : RWReg<u32>,        /* NAND Flash control register 2,                       Address offset: 0x60 */
  pub sr2        : RWReg<u32>,        /* NAND Flash FIFO status and interrupt register 2,     Address offset: 0x64 */
  pub pmem2      : RWReg<u32>,        /* NAND Flash Common memory space timing register 2,    Address offset: 0x68 */
  pub patt2      : RWReg<u32>,        /* NAND Flash Attribute memory space timing register 2, Address offset: 0x6C */
      reserved0  : u32,               /* Reserved, 0x70                                                            */
  pub eccr2      : RWReg<u32>,        /* NAND Flash ECC result registers 2,                   Address offset: 0x74 */
}/* 0x0018 */

#[repr(C, packed)]
pub struct FSMC_Bank3_Type {
  pub pcr3       : RWReg<u32>,        /* NAND Flash control register 3,                       Address offset: 0x80 */
  pub sr3        : RWReg<u32>,        /* NAND Flash FIFO status and interrupt register 3,     Address offset: 0x84 */
  pub pmem3      : RWReg<u32>,        /* NAND Flash Common memory space timing register 3,    Address offset: 0x88 */
  pub patt3      : RWReg<u32>,        /* NAND Flash Attribute memory space timing register 3, Address offset: 0x8C */
      reserved0  : u32,               /* Reserved, 0x90                                                            */
  pub eccr3      : RWReg<u32>,        /* NAND Flash ECC result registers 3,                   Address offset: 0x94 */
}/* 0x0018 */

#[repr(C, packed)]
pub struct FSMC_Bank4_Type {
  pub pcr4       : RWReg<u32>,        /* PC Card  control register 4,                       Address offset: 0xA0 */
  pub sr4        : RWReg<u32>,        /* PC Card  FIFO status and interrupt register 4,     Address offset: 0xA4 */
  pub pmem4      : RWReg<u32>,        /* PC Card  Common memory space timing register 4,    Address offset: 0xA8 */
  pub patt4      : RWReg<u32>,        /* PC Card  Attribute memory space timing register 4, Address offset: 0xAC */
  pub pio4       : RWReg<u32>,        /* PC Card  I/O space timing register 4,              Address offset: 0xB0 */
}/* 0x0014 */

#[repr(C, packed)]
pub struct GPIO_Type {
  pub moder      : RWReg<u32>,        /* GPIO port mode register,               Address offset: 0x00      */
  pub otyper     : RWReg<u32>,        /* GPIO port output type register,        Address offset: 0x04      */
  pub ospeedr    : RWReg<u32>,        /* GPIO port output speed register,       Address offset: 0x08      */
  pub pupdr      : RWReg<u32>,        /* GPIO port pull-up/pull-down register,  Address offset: 0x0C      */
  pub idr        : RWReg<u32>,        /* GPIO port input data register,         Address offset: 0x10      */
  pub odr        : RWReg<u32>,        /* GPIO port output data register,        Address offset: 0x14      */
  pub bsrrl      : RWReg<u16>,        /* GPIO port bit set/reset low register,  Address offset: 0x18      */
  pub bsrrh      : RWReg<u16>,        /* GPIO port bit set/reset high register, Address offset: 0x1A      */
  pub lckr       : RWReg<u32>,        /* GPIO port configuration lock register, Address offset: 0x1C      */
  pub afr        :[RWReg<u32>; 2],    /* GPIO alternate function registers,     Address offset: 0x20-0x24 */
}/* 0x0028 */

#[repr(C, packed)]
pub struct SYSCFG_Type {
  pub memrmp     : RWReg<u32>,        /* SYSCFG memory remap register,                      Address offset: 0x00      */
  pub pmc        : RWReg<u32>,        /* SYSCFG peripheral mode configuration register,     Address offset: 0x04      */
  pub exticr     :[RWReg<u32>; 4],    /* SYSCFG external interrupt configuration registers, Address offset: 0x08-0x14 */
      reserved   :[u32; 2],           /* Reserved, 0x18-0x1C                                                          */
  pub cmpcr      : RWReg<u32>,        /* SYSCFG Compensation cell control register,         Address offset: 0x20      */
}/* 0x0024 */

#[repr(C, packed)]
pub struct I2C_Type {
  pub cr1        : RWReg<u16>,        /* I2C Control register 1,     Address offset: 0x00 */
      reserved0  : u16,               /* Reserved, 0x02                                   */
  pub cr2        : RWReg<u16>,        /* I2C Control register 2,     Address offset: 0x04 */
      reserved1  : u16,               /* Reserved, 0x06                                   */
  pub oar1       : RWReg<u16>,        /* I2C Own address register 1, Address offset: 0x08 */
      reserved2  : u16,               /* Reserved, 0x0A                                   */
  pub oar2       : RWReg<u16>,        /* I2C Own address register 2, Address offset: 0x0C */
      reserved3  : u16,               /* Reserved, 0x0E                                   */
  pub dr         : RWReg<u16>,        /* I2C Data register,          Address offset: 0x10 */
      reserved4  : u16,               /* Reserved, 0x12                                   */
  pub sr1        : RWReg<u16>,        /* I2C Status register 1,      Address offset: 0x14 */
      reserved5  : u16,               /* Reserved, 0x16                                   */
  pub sr2        : RWReg<u16>,        /* I2C Status register 2,      Address offset: 0x18 */
      reserved6  : u16,               /* Reserved, 0x1A                                   */
  pub ccr        : RWReg<u16>,        /* I2C Clock control register, Address offset: 0x1C */
      reserved7  : u16,               /* Reserved, 0x1E                                   */
  pub trise      : RWReg<u16>,        /* I2C TRISE register,         Address offset: 0x20 */
      reserved8  : u16,               /* Reserved, 0x22                                   */
}/* 0x0024 */

#[repr(C, packed)]
pub struct IWDG_Type {
  pub kr         : RWReg<u32>,        /* IWDG Key register,       Address offset: 0x00 */
  pub pr         : RWReg<u32>,        /* IWDG Prescaler register, Address offset: 0x04 */
  pub rlr        : RWReg<u32>,        /* IWDG Reload register,    Address offset: 0x08 */
  pub sr         : RWReg<u32>,        /* IWDG Status register,    Address offset: 0x0C */
}/* 0x0010 */

#[repr(C, packed)]
pub struct PWR_Type {
  pub cr         : RWReg<u32>,        /* PWR power control register,        Address offset: 0x00 */
  pub csr        : RWReg<u32>,        /* PWR power control/status register, Address offset: 0x04 */
}/* 0x0008 */

#[repr(C, packed)]
pub struct RCC_Type {
  pub cr         : RWReg<u32>,        /* RCC clock control register,                                  Address offset: 0x00 */
  pub pllcfgr    : RWReg<u32>,        /* RCC PLL configuration register,                              Address offset: 0x04 */
  pub cfgr       : RWReg<u32>,        /* RCC clock configuration register,                            Address offset: 0x08 */
  pub cir        : RWReg<u32>,        /* RCC clock interrupt register,                                Address offset: 0x0C */
  pub ahb1rstr   : RWReg<u32>,        /* RCC AHB1 peripheral reset register,                          Address offset: 0x10 */
  pub ahb2rstr   : RWReg<u32>,        /* RCC AHB2 peripheral reset register,                          Address offset: 0x14 */
  pub ahb3rstr   : RWReg<u32>,        /* RCC AHB3 peripheral reset register,                          Address offset: 0x18 */
      reserved0  : u32,               /* Reserved, 0x1C                                                                    */
  pub apb1rstr   : RWReg<u32>,        /* RCC APB1 peripheral reset register,                          Address offset: 0x20 */
  pub apb2rstr   : RWReg<u32>,        /* RCC APB2 peripheral reset register,                          Address offset: 0x24 */
      reserved1  :[u32; 2],           /* Reserved, 0x28-0x2C                                                               */
  pub ahb1enr    : RWReg<u32>,        /* RCC AHB1 peripheral clock register,                          Address offset: 0x30 */
  pub ahb2enr    : RWReg<u32>,        /* RCC AHB2 peripheral clock register,                          Address offset: 0x34 */
  pub ahb3enr    : RWReg<u32>,        /* RCC AHB3 peripheral clock register,                          Address offset: 0x38 */
      reserved2  : u32,               /* Reserved, 0x3C                                                                    */
  pub apb1enr    : RWReg<u32>,        /* RCC APB1 peripheral clock enable register,                   Address offset: 0x40 */
  pub apb2enr    : RWReg<u32>,        /* RCC APB2 peripheral clock enable register,                   Address offset: 0x44 */
      reserved3  :[u32; 2],           /* Reserved, 0x48-0x4C                                                               */
  pub ahb1lpenr  : RWReg<u32>,        /* RCC AHB1 peripheral clock enable in low power mode register, Address offset: 0x50 */
  pub ahb2lpenr  : RWReg<u32>,        /* RCC AHB2 peripheral clock enable in low power mode register, Address offset: 0x54 */
  pub ahb3lpenr  : RWReg<u32>,        /* RCC AHB3 peripheral clock enable in low power mode register, Address offset: 0x58 */
      reserved4  : u32,               /* Reserved, 0x5C                                                                    */
  pub apb1lpenr  : RWReg<u32>,        /* RCC APB1 peripheral clock enable in low power mode register, Address offset: 0x60 */
  pub apb2lpenr  : RWReg<u32>,        /* RCC APB2 peripheral clock enable in low power mode register, Address offset: 0x64 */
      reserved5  :[u32; 2],           /* Reserved, 0x68-0x6C                                                               */
  pub bdcr       : RWReg<u32>,        /* RCC Backup domain control register,                          Address offset: 0x70 */
  pub csr        : RWReg<u32>,        /* RCC clock control & status register,                         Address offset: 0x74 */
      reserved6  :[u32; 2],           /* Reserved, 0x78-0x7C                                                               */
  pub sscgr      : RWReg<u32>,        /* RCC spread spectrum clock generation register,               Address offset: 0x80 */
  pub plli2scfgr : RWReg<u32>,        /* RCC PLLI2S configuration register,                           Address offset: 0x84 */
}/* 0x0088 */

#[repr(C, packed)]
pub struct RTC_Type {
  pub tr         : RWReg<u32>,        /* RTC time register,                                        Address offset: 0x00 */
  pub dr         : RWReg<u32>,        /* RTC date register,                                        Address offset: 0x04 */
  pub cr         : RWReg<u32>,        /* RTC control register,                                     Address offset: 0x08 */
  pub isr        : RWReg<u32>,        /* RTC initialization and status register,                   Address offset: 0x0C */
  pub prer       : RWReg<u32>,        /* RTC prescaler register,                                   Address offset: 0x10 */
  pub wutr       : RWReg<u32>,        /* RTC wakeup timer register,                                Address offset: 0x14 */
  pub calibr     : RWReg<u32>,        /* RTC calibration register,                                 Address offset: 0x18 */
  pub alrmar     : RWReg<u32>,        /* RTC alarm A register,                                     Address offset: 0x1C */
  pub alrmbr     : RWReg<u32>,        /* RTC alarm B register,                                     Address offset: 0x20 */
  pub wpr        : RWReg<u32>,        /* RTC write protection register,                            Address offset: 0x24 */
  pub ssr        : RWReg<u32>,        /* RTC sub second register,                                  Address offset: 0x28 */
  pub shiftr     : RWReg<u32>,        /* RTC shift control register,                               Address offset: 0x2C */
  pub tstr       : RWReg<u32>,        /* RTC time stamp time register,                             Address offset: 0x30 */
  pub tsdr       : RWReg<u32>,        /* RTC time stamp date register,                             Address offset: 0x34 */
  pub tsssr      : RWReg<u32>,        /* RTC time-stamp sub second register,                       Address offset: 0x38 */
  pub calr       : RWReg<u32>,        /* RTC calibration register,                                 Address offset: 0x3C */
  pub tafcr      : RWReg<u32>,        /* RTC tamper and alternate function configuration register, Address offset: 0x40 */
  pub alrmassr   : RWReg<u32>,        /* RTC alarm A sub second register,                          Address offset: 0x44 */
  pub alrmbssr   : RWReg<u32>,        /* RTC alarm B sub second register,                          Address offset: 0x48 */
      reserved7  : u32,               /* Reserved, 0x4C                                                                 */
  pub bkp0r      : RWReg<u32>,        /* RTC backup register 1,                                    Address offset: 0x50 */
  pub bkp1r      : RWReg<u32>,        /* RTC backup register 1,                                    Address offset: 0x54 */
  pub bkp2r      : RWReg<u32>,        /* RTC backup register 2,                                    Address offset: 0x58 */
  pub bkp3r      : RWReg<u32>,        /* RTC backup register 3,                                    Address offset: 0x5C */
  pub bkp4r      : RWReg<u32>,        /* RTC backup register 4,                                    Address offset: 0x60 */
  pub bkp5r      : RWReg<u32>,        /* RTC backup register 5,                                    Address offset: 0x64 */
  pub bkp6r      : RWReg<u32>,        /* RTC backup register 6,                                    Address offset: 0x68 */
  pub bkp7r      : RWReg<u32>,        /* RTC backup register 7,                                    Address offset: 0x6C */
  pub bkp8r      : RWReg<u32>,        /* RTC backup register 8,                                    Address offset: 0x70 */
  pub bkp9r      : RWReg<u32>,        /* RTC backup register 9,                                    Address offset: 0x74 */
  pub bkp10r     : RWReg<u32>,        /* RTC backup register 10,                                   Address offset: 0x78 */
  pub bkp11r     : RWReg<u32>,        /* RTC backup register 11,                                   Address offset: 0x7C */
  pub bkp12r     : RWReg<u32>,        /* RTC backup register 12,                                   Address offset: 0x80 */
  pub bkp13r     : RWReg<u32>,        /* RTC backup register 13,                                   Address offset: 0x84 */
  pub bkp14r     : RWReg<u32>,        /* RTC backup register 14,                                   Address offset: 0x88 */
  pub bkp15r     : RWReg<u32>,        /* RTC backup register 15,                                   Address offset: 0x8C */
  pub bkp16r     : RWReg<u32>,        /* RTC backup register 16,                                   Address offset: 0x90 */
  pub bkp17r     : RWReg<u32>,        /* RTC backup register 17,                                   Address offset: 0x94 */
  pub bkp18r     : RWReg<u32>,        /* RTC backup register 18,                                   Address offset: 0x98 */
  pub bkp19r     : RWReg<u32>,        /* RTC backup register 19,                                   Address offset: 0x9C */
}/* 0x00A0 */

#[repr(C, packed)]
pub struct SDIO_Type {
  pub power      : RWReg<u32>,        /* SDIO power control register,    Address offset: 0x00 */
  pub clkcr      : RWReg<u32>,        /* SDI clock control register,     Address offset: 0x04 */
  pub arg        : RWReg<u32>,        /* SDIO argument register,         Address offset: 0x08 */
  pub cmd        : RWReg<u32>,        /* SDIO command register,          Address offset: 0x0C */
  pub respcmd    : ROReg<u32>,        /* SDIO command response register, Address offset: 0x10 */
  pub resp1      : ROReg<u32>,        /* SDIO response 1 register,       Address offset: 0x14 */
  pub resp2      : ROReg<u32>,        /* SDIO response 2 register,       Address offset: 0x18 */
  pub resp3      : ROReg<u32>,        /* SDIO response 3 register,       Address offset: 0x1C */
  pub resp4      : ROReg<u32>,        /* SDIO response 4 register,       Address offset: 0x20 */
  pub dtimer     : RWReg<u32>,        /* SDIO data timer register,       Address offset: 0x24 */
  pub dlen       : RWReg<u32>,        /* SDIO data length register,      Address offset: 0x28 */
  pub dctrl      : RWReg<u32>,        /* SDIO data control register,     Address offset: 0x2C */
  pub dcount     : ROReg<u32>,        /* SDIO data counter register,     Address offset: 0x30 */
  pub sta        : ROReg<u32>,        /* SDIO status register,           Address offset: 0x34 */
  pub icr        : RWReg<u32>,        /* SDIO interrupt clear register,  Address offset: 0x38 */
  pub mask       : RWReg<u32>,        /* SDIO mask register,             Address offset: 0x3C */
      reserved0  :[u32; 2],           /* Reserved, 0x40-0x44                                  */
  pub fifocnt    : ROReg<u32>,        /* SDIO FIFO counter register,     Address offset: 0x48 */
      reserved1  :[u32; 13],          /* Reserved, 0x4C-0x7C                                  */
  pub fifo       : RWReg<u32>,        /* SDIO data FIFO register,        Address offset: 0x80 */
}/* 0x0084 */

#[repr(C, packed)]
pub struct SPI_Type {
  pub cr1        : RWReg<u16>,        /* SPI control register 1 (not used in I2S mode),      Address offset: 0x00 */
      reserved0  : u16,               /* Reserved, 0x02                                                           */
  pub cr2        : RWReg<u16>,        /* SPI control register 2,                             Address offset: 0x04 */
      reserved1  : u16,               /* Reserved, 0x06                                                           */
  pub sr         : RWReg<u16>,        /* SPI status register,                                Address offset: 0x08 */
      reserved2  : u16,               /* Reserved, 0x0A                                                           */
  pub dr         : RWReg<u16>,        /* SPI data register,                                  Address offset: 0x0C */
      reserved3  : u16,               /* Reserved, 0x0E                                                           */
  pub crcpr      : RWReg<u16>,        /* SPI CRC polynomial register (not used in I2S mode), Address offset: 0x10 */
      reserved4  : u16,               /* Reserved, 0x12                                                           */
  pub rxcrcr     : RWReg<u16>,        /* SPI RX CRC register (not used in I2S mode),         Address offset: 0x14 */
      reserved5  : u16,               /* Reserved, 0x16                                                           */
  pub txcrcr     : RWReg<u16>,        /* SPI TX CRC register (not used in I2S mode),         Address offset: 0x18 */
      reserved6  : u16,               /* Reserved, 0x1A                                                           */
  pub i2scfgr    : RWReg<u16>,        /* SPI_I2S configuration register,                     Address offset: 0x1C */
      reserved7  : u16,               /* Reserved, 0x1E                                                           */
  pub i2spr      : RWReg<u16>,        /* SPI_I2S prescaler register,                         Address offset: 0x20 */
      reserved8  : u16,               /* Reserved, 0x22                                                           */
}/* 0x0024 */

#[repr(C, packed)]
pub struct TIM_Type {
  pub cr1        : RWReg<u16>,        /* TIM control register 1,              Address offset: 0x00 */
      reserved0  : u16,               /* Reserved, 0x02                                            */
  pub cr2        : RWReg<u16>,        /* TIM control register 2,              Address offset: 0x04 */
      reserved1  : u16,               /* Reserved, 0x06                                            */
  pub smcr       : RWReg<u16>,        /* TIM slave mode control register,     Address offset: 0x08 */
      reserved2  : u16,               /* Reserved, 0x0A                                            */
  pub dier       : RWReg<u16>,        /* TIM DMA/interrupt enable register,   Address offset: 0x0C */
      reserved3  : u16,               /* Reserved, 0x0E                                            */
  pub sr         : RWReg<u16>,        /* TIM status register,                 Address offset: 0x10 */
      reserved4  : u16,               /* Reserved, 0x12                                            */
  pub egr        : RWReg<u16>,        /* TIM event generation register,       Address offset: 0x14 */
      reserved5  : u16,               /* Reserved, 0x16                                            */
  pub ccmr1      : RWReg<u16>,        /* TIM capture/compare mode register 1, Address offset: 0x18 */
      reserved6  : u16,               /* Reserved, 0x1A                                            */
  pub ccmr2      : RWReg<u16>,        /* TIM capture/compare mode register 2, Address offset: 0x1C */
      reserved7  : u16,               /* Reserved, 0x1E                                            */
  pub ccer       : RWReg<u16>,        /* TIM capture/compare enable register, Address offset: 0x20 */
      reserved8  : u16,               /* Reserved, 0x22                                            */
  pub cnt        : RWReg<u32>,        /* TIM counter register,                Address offset: 0x24 */
  pub psc        : RWReg<u16>,        /* TIM prescaler,                       Address offset: 0x28 */
      reserved9  : u16,               /* Reserved, 0x2A                                            */
  pub arr        : RWReg<u32>,        /* TIM auto-reload register,            Address offset: 0x2C */
  pub rcr        : RWReg<u16>,        /* TIM repetition counter register,     Address offset: 0x30 */
      reserved10 : u16,               /* Reserved, 0x32                                            */
  pub ccr1       : RWReg<u32>,        /* TIM capture/compare register 1,      Address offset: 0x34 */
  pub ccr2       : RWReg<u32>,        /* TIM capture/compare register 2,      Address offset: 0x38 */
  pub ccr3       : RWReg<u32>,        /* TIM capture/compare register 3,      Address offset: 0x3C */
  pub ccr4       : RWReg<u32>,        /* TIM capture/compare register 4,      Address offset: 0x40 */
  pub bdtr       : RWReg<u16>,        /* TIM break and dead-time register,    Address offset: 0x44 */
      reserved11 : u16,               /* Reserved, 0x46                                            */
  pub dcr        : RWReg<u16>,        /* TIM DMA control register,            Address offset: 0x48 */
      reserved12 : u16,               /* Reserved, 0x4A                                            */
  pub dmar       : RWReg<u16>,        /* TIM DMA address for full transfer,   Address offset: 0x4C */
      reserved13 : u16,               /* Reserved, 0x4E                                            */
  pub or         : RWReg<u16>,        /* TIM option register,                 Address offset: 0x50 */
      reserved14 : u16,               /* Reserved, 0x52                                            */
}/* 0x0054 */

#[repr(C, packed)]
pub struct USART_Type {
  pub sr         : RWReg<u16>,        /* USART Status register,                   Address offset: 0x00 */
      reserved0  : u16,               /* Reserved, 0x02                                                */
  pub dr         : RWReg<u16>,        /* USART Data register,                     Address offset: 0x04 */
      reserved1  : u16,               /* Reserved, 0x06                                                */
  pub brr        : RWReg<u16>,        /* USART Baud rate register,                Address offset: 0x08 */
      reserved2  : u16,               /* Reserved, 0x0A                                                */
  pub cr1        : RWReg<u16>,        /* USART Control register 1,                Address offset: 0x0C */
      reserved3  : u16,               /* Reserved, 0x0E                                                */
  pub cr2        : RWReg<u16>,        /* USART Control register 2,                Address offset: 0x10 */
      reserved4  : u16,               /* Reserved, 0x12                                                */
  pub cr3        : RWReg<u16>,        /* USART Control register 3,                Address offset: 0x14 */
      reserved5  : u16,               /* Reserved, 0x16                                                */
  pub gtpr       : RWReg<u16>,        /* USART Guard time and prescaler register, Address offset: 0x18 */
      reserved6  : u16,               /* Reserved, 0x1A                                                */
}/* 0x001C */

#[repr(C, packed)]
pub struct WWDG_Type {
  pub cr         : RWReg<u32>,        /* WWDG Control register,       Address offset: 0x00 */
  pub cfr        : RWReg<u32>,        /* WWDG Configuration register, Address offset: 0x04 */
  pub sr         : RWReg<u32>,        /* WWDG Status register,        Address offset: 0x08 */
}/* 0x000C */

#[repr(C, packed)]
pub struct CRYP_Type {
  pub cr         : RWReg<u32>,        /* CRYP control register,                            Address offset: 0x00 */
  pub sr         : RWReg<u32>,        /* CRYP status register,                             Address offset: 0x04 */
  pub dr         : RWReg<u32>,        /* CRYP data input register,                         Address offset: 0x08 */
  pub dout       : RWReg<u32>,        /* CRYP data output register,                        Address offset: 0x0C */
  pub dmacr      : RWReg<u32>,        /* CRYP DMA control register,                        Address offset: 0x10 */
  pub imscr      : RWReg<u32>,        /* CRYP interrupt mask set/clear register,           Address offset: 0x14 */
  pub risr       : RWReg<u32>,        /* CRYP raw interrupt status register,               Address offset: 0x18 */
  pub misr       : RWReg<u32>,        /* CRYP masked interrupt status register,            Address offset: 0x1C */
  pub k0lr       : RWReg<u32>,        /* CRYP key left  register 0,                        Address offset: 0x20 */
  pub k0rr       : RWReg<u32>,        /* CRYP key right register 0,                        Address offset: 0x24 */
  pub k1lr       : RWReg<u32>,        /* CRYP key left  register 1,                        Address offset: 0x28 */
  pub k1rr       : RWReg<u32>,        /* CRYP key right register 1,                        Address offset: 0x2C */
  pub k2lr       : RWReg<u32>,        /* CRYP key left  register 2,                        Address offset: 0x30 */
  pub k2rr       : RWReg<u32>,        /* CRYP key right register 2,                        Address offset: 0x34 */
  pub k3lr       : RWReg<u32>,        /* CRYP key left  register 3,                        Address offset: 0x38 */
  pub k3rr       : RWReg<u32>,        /* CRYP key right register 3,                        Address offset: 0x3C */
  pub iv0lr      : RWReg<u32>,        /* CRYP initialization vector left-word  register 0, Address offset: 0x40 */
  pub iv0rr      : RWReg<u32>,        /* CRYP initialization vector right-word register 0, Address offset: 0x44 */
  pub iv1lr      : RWReg<u32>,        /* CRYP initialization vector left-word  register 1, Address offset: 0x48 */
  pub iv1rr      : RWReg<u32>,        /* CRYP initialization vector right-word register 1, Address offset: 0x4C */
}/* 0x0050 */

#[repr(C, packed)]
pub struct HASH_Type {
  pub cr         : RWReg<u32>,        /* HASH control register,          Address offset: 0x00        */
  pub din        : RWReg<u32>,        /* HASH data input register,       Address offset: 0x04        */
  pub str        : RWReg<u32>,        /* HASH start register,            Address offset: 0x08        */
  pub hr         :[RWReg<u32>; 5],    /* HASH digest registers,          Address offset: 0x0C-0x1C   */
  pub imr        : RWReg<u32>,        /* HASH interrupt enable register, Address offset: 0x20        */
  pub sr         : RWReg<u32>,        /* HASH status register,           Address offset: 0x24        */
      reserved   :[u32; 52],          /* Reserved, 0x28-0xF4                                         */
  pub csr        :[RWReg<u32>; 51],   /* HASH context swap registers,    Address offset: 0x0F8-0x1C0 */
}/* 0x01C4 */

#[repr(C, packed)]
pub struct RNG_Type {
  pub cr         : RWReg<u32>,        /* RNG control register, Address offset: 0x00 */
  pub sr         : RWReg<u32>,        /* RNG status register,  Address offset: 0x04 */
  pub dr         : RWReg<u32>,        /* RNG data register,    Address offset: 0x08 */
}/* 0x000C */
#[repr(C, packed)]
pub struct STM32F4Device {
  pub tim2     : TIM_Type,        /* 0x40000000, w=<0x0054> */
  padding01    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim3     : TIM_Type,        /* 0x40000400, w=<0x0054> */
  padding02    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim4     : TIM_Type,        /* 0x40000800, w=<0x0054> */
  padding03    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim5     : TIM_Type,        /* 0x40000C00, w=<0x0054> */
  padding04    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim6     : TIM_Type,        /* 0x40001000, w=<0x0054> */
  padding05    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim7     : TIM_Type,        /* 0x40001400, w=<0x0054> */
  padding06    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim12    : TIM_Type,        /* 0x40001800, w=<0x0054> */
  padding07    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim13    : TIM_Type,        /* 0x40001C00, w=<0x0054> */
  padding08    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim14    : TIM_Type,        /* 0x40002000, w=<0x0054> */
  padding09    : [u32; 0x01EB],   /*             w=<0x07AC> */
  pub rtc      : RTC_Type,        /* 0x40002800, w=<0x00A0> */
  padding10    : [u32; 0x00D8],   /*             w=<0x0360> */
  pub wwdg     : WWDG_Type,       /* 0x40002C00, w=<0x000C> */
  padding11    : [u32; 0x00FD],   /*             w=<0x03F4> */
  pub iwdg     : IWDG_Type,       /* 0x40003000, w=<0x0010> */
  padding12    : [u32; 0x00FC],   /*             w=<0x03F0> */
  pub i2s2ext  : SPI_Type,        /* 0x40003400, w=<0x0024> */
  padding13    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub spi2     : SPI_Type,        /* 0x40003800, w=<0x0024> */
  padding14    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub spi3     : SPI_Type,        /* 0x40003C00, w=<0x0024> */
  padding15    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub i2s3ext  : SPI_Type,        /* 0x40004000, w=<0x0024> */
  padding16    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub usart2   : USART_Type,      /* 0x40004400, w=<0x001C> */
  padding17    : [u32; 0x00F9],   /*             w=<0x03E4> */
  pub usart3   : USART_Type,      /* 0x40004800, w=<0x001C> */
  padding18    : [u32; 0x00F9],   /*             w=<0x03E4> */
  pub uart4    : USART_Type,      /* 0x40004C00, w=<0x001C> */
  padding19    : [u32; 0x00F9],   /*             w=<0x03E4> */
  pub uart5    : USART_Type,      /* 0x40005000, w=<0x001C> */
  padding20    : [u32; 0x00F9],   /*             w=<0x03E4> */
  pub i2c1     : I2C_Type,        /* 0x40005400, w=<0x0024> */
  padding21    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub i2c2     : I2C_Type,        /* 0x40005800, w=<0x0024> */
  padding22    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub i2c3     : I2C_Type,        /* 0x40005C00, w=<0x0024> */
  padding23    : [u32; 0x01F7],   /*             w=<0x07DC> */
  pub can1     : CAN_Type,        /* 0x40006400, w=<0x0320> */
  padding24    : [u32; 0x0038],   /*             w=<0x00E0> */
  pub can2     : CAN_Type,        /* 0x40006800, w=<0x0320> */
  padding25    : [u32; 0x0138],   /*             w=<0x04E0> */
  pub pwr      : PWR_Type,        /* 0x40007000, w=<0x0008> */
  padding26    : [u32; 0x00FE],   /*             w=<0x03F8> */
  pub dac      : DAC_Type,        /* 0x40007400, w=<0x0038> */
  padding27    : [u32; 0x22F2],   /*             w=<0x8BC8> */
  pub tim1     : TIM_Type,        /* 0x40010000, w=<0x0054> */
  padding28    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim8     : TIM_Type,        /* 0x40010400, w=<0x0054> */
  padding29    : [u32; 0x02EB],   /*             w=<0x0BAC> */
  pub usart1   : USART_Type,      /* 0x40011000, w=<0x001C> */
  padding30    : [u32; 0x00F9],   /*             w=<0x03E4> */
  pub usart6   : USART_Type,      /* 0x40011400, w=<0x001C> */
  padding31    : [u32; 0x02F9],   /*             w=<0x0BE4> */
  pub adc1     : ADC_Type,        /* 0x40012000, w=<0x0050> */
  padding32    : [u32; 0x002C],   /*             w=<0x00B0> */
  pub adc2     : ADC_Type,        /* 0x40012100, w=<0x0050> */
  padding33    : [u32; 0x002C],   /*             w=<0x00B0> */
  pub adc3     : ADC_Type,        /* 0x40012200, w=<0x0050> */
  padding34    : [u32; 0x002C],   /*             w=<0x00B0> */
  pub adc      : ADC_Common_Type, /* 0x40012300, w=<0x000C> */
  padding35    : [u32; 0x023D],   /*             w=<0x08F4> */
  pub sdio     : SDIO_Type,       /* 0x40012C00, w=<0x0084> */
  padding36    : [u32; 0x00DF],   /*             w=<0x037C> */
  pub spi1     : SPI_Type,        /* 0x40013000, w=<0x0024> */
  padding37    : [u32; 0x01F7],   /*             w=<0x07DC> */
  pub syscfg   : SYSCFG_Type,     /* 0x40013800, w=<0x0024> */
  padding38    : [u32; 0x00F7],   /*             w=<0x03DC> */
  pub exti     : EXTI_Type,       /* 0x40013C00, w=<0x0018> */
  padding39    : [u32; 0x00FA],   /*             w=<0x03E8> */
  pub tim9     : TIM_Type,        /* 0x40014000, w=<0x0054> */
  padding40    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim10    : TIM_Type,        /* 0x40014400, w=<0x0054> */
  padding41    : [u32; 0x00EB],   /*             w=<0x03AC> */
  pub tim11    : TIM_Type,        /* 0x40014800, w=<0x0054> */
  padding42    : [u32; 0x2DEB],   /*             w=<0xB7AC> */
  pub gpioa    : GPIO_Type,       /* 0x40020000, w=<0x0028> */
  padding43    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpiob    : GPIO_Type,       /* 0x40020400, w=<0x0028> */
  padding44    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpioc    : GPIO_Type,       /* 0x40020800, w=<0x0028> */
  padding45    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpiod    : GPIO_Type,       /* 0x40020C00, w=<0x0028> */
  padding46    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpioe    : GPIO_Type,       /* 0x40021000, w=<0x0028> */
  padding47    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpiof    : GPIO_Type,       /* 0x40021400, w=<0x0028> */
  padding48    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpiog    : GPIO_Type,       /* 0x40021800, w=<0x0028> */
  padding49    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpioh    : GPIO_Type,       /* 0x40021C00, w=<0x0028> */
  padding50    : [u32; 0x00F6],   /*             w=<0x03D8> */
  pub gpioi    : GPIO_Type,       /* 0x40022000, w=<0x0028> */
  padding51    : [u32; 0x03F6],   /*             w=<0x0FD8> */
  pub crc      : CRC_Type,        /* 0x40023000, w=<0x000C> */
  padding52    : [u32; 0x01FD],   /*             w=<0x07F4> */
  pub rcc      : RCC_Type,        /* 0x40023800, w=<0x0088> */
  padding53    : [u32; 0x00DE],   /*             w=<0x0378> */
  pub flash    : FLASH_Type,      /* 0x40023C00, w=<0x0018> */
  padding54    : [u32; 0x08FA],   /*             w=<0x23E8> */
  pub dma1     : DMA_Type,        /* 0x40026000, w=<0x0010> */
  pub dma1_stream0 : DMA_Stream_Type, /* 0x40026010, w=<0x0018> */
  pub dma1_stream1 : DMA_Stream_Type, /* 0x40026028, w=<0x0018> */
  pub dma1_stream2 : DMA_Stream_Type, /* 0x40026040, w=<0x0018> */
  pub dma1_stream3 : DMA_Stream_Type, /* 0x40026058, w=<0x0018> */
  pub dma1_stream4 : DMA_Stream_Type, /* 0x40026070, w=<0x0018> */
  pub dma1_stream5 : DMA_Stream_Type, /* 0x40026088, w=<0x0018> */
  pub dma1_stream6 : DMA_Stream_Type, /* 0x400260A0, w=<0x0018> */
  pub dma1_stream7 : DMA_Stream_Type, /* 0x400260B8, w=<0x0018> */
  padding63    : [u32; 0x00CC],   /*             w=<0x0330> */
  pub dma2     : DMA_Type,        /* 0x40026400, w=<0x0010> */
  pub dma2_stream0 : DMA_Stream_Type, /* 0x40026410, w=<0x0018> */
  pub dma2_stream1 : DMA_Stream_Type, /* 0x40026428, w=<0x0018> */
  pub dma2_stream2 : DMA_Stream_Type, /* 0x40026440, w=<0x0018> */
  pub dma2_stream3 : DMA_Stream_Type, /* 0x40026458, w=<0x0018> */
  pub dma2_stream4 : DMA_Stream_Type, /* 0x40026470, w=<0x0018> */
  pub dma2_stream5 : DMA_Stream_Type, /* 0x40026488, w=<0x0018> */
  pub dma2_stream6 : DMA_Stream_Type, /* 0x400264A0, w=<0x0018> */
  pub dma2_stream7 : DMA_Stream_Type, /* 0x400264B8, w=<0x0018> */
  padding72    : [u32; 0x06CC],   /*             w=<0x1B30> */
  pub eth      : ETH_Type,        /* 0x40028000, w=<0x1058> */
}
pub const DCMI          : *const DCMI_Type         = 0x50050000u32 as *const DCMI_Type;
pub const CRYP          : *const CRYP_Type         = 0x50060000u32 as *const CRYP_Type;
pub const HASH          : *const HASH_Type         = 0x50060400u32 as *const HASH_Type;
pub const RNG           : *const RNG_Type          = 0x50060800u32 as *const RNG_Type;
pub const FSMC_BANK1    : *const FSMC_Bank1_Type   = 0xA0000000u32 as *const FSMC_Bank1_Type;
pub const FSMC_BANK2    : *const FSMC_Bank2_Type   = 0xA0000060u32 as *const FSMC_Bank2_Type;
pub const FSMC_BANK3    : *const FSMC_Bank3_Type   = 0xA0000080u32 as *const FSMC_Bank3_Type;
pub const FSMC_BANK4    : *const FSMC_Bank4_Type   = 0xA00000A0u32 as *const FSMC_Bank4_Type;
pub const FSMC_BANK1E   : *const FSMC_Bank1E_Type  = 0xA0000104u32 as *const FSMC_Bank1E_Type;
pub const ITM           : *const ITM_Type          = 0xE0000000u32 as *const ITM_Type;
pub const SCNSCB        : *const SCnSCB_Type       = 0xE000E000u32 as *const SCnSCB_Type;
pub const SYSTICK       : *const SysTick_Type      = 0xE000E010u32 as *const SysTick_Type;
pub const NVIC          : *const NVIC_Type         = 0xE000E100u32 as *const NVIC_Type;
pub const SCB           : *const SCB_Type          = 0xE000ED00u32 as *const SCB_Type;
pub const COREDEBUG     : *const CoreDebug_Type    = 0xE000EDF0u32 as *const CoreDebug_Type;
pub const DBGMCU        : *const DBGMCU_Type       = 0xE0042000u32 as *const DBGMCU_Type;

#[link_section=".device_reg"]  /* hodne divny hack */
pub static STM : STM32F4Device = STM32F4Device {
  tim2 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding01: [0; 0x00EB],
  tim3 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding02: [0; 0x00EB],
  tim4 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding03: [0; 0x00EB],
  tim5 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding04: [0; 0x00EB],
  tim6 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding05: [0; 0x00EB],
  tim7 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding06: [0; 0x00EB],
  tim12 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding07: [0; 0x00EB],
  tim13 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding08: [0; 0x00EB],
  tim14 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding09: [0; 0x01EB],
  rtc : RTC_Type {
    tr : RWReg {
        value : 0,
       },
    dr : RWReg {
        value : 0,
       },
    cr : RWReg {
        value : 0,
       },
    isr : RWReg {
        value : 0,
       },
    prer : RWReg {
        value : 0,
       },
    wutr : RWReg {
        value : 0,
       },
    calibr : RWReg {
        value : 0,
       },
    alrmar : RWReg {
        value : 0,
       },
    alrmbr : RWReg {
        value : 0,
       },
    wpr : RWReg {
        value : 0,
       },
    ssr : RWReg {
        value : 0,
       },
    shiftr : RWReg {
        value : 0,
       },
    tstr : RWReg {
        value : 0,
       },
    tsdr : RWReg {
        value : 0,
       },
    tsssr : RWReg {
        value : 0,
       },
    calr : RWReg {
        value : 0,
       },
    tafcr : RWReg {
        value : 0,
       },
    alrmassr : RWReg {
        value : 0,
       },
    alrmbssr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    bkp0r : RWReg {
        value : 0,
       },
    bkp1r : RWReg {
        value : 0,
       },
    bkp2r : RWReg {
        value : 0,
       },
    bkp3r : RWReg {
        value : 0,
       },
    bkp4r : RWReg {
        value : 0,
       },
    bkp5r : RWReg {
        value : 0,
       },
    bkp6r : RWReg {
        value : 0,
       },
    bkp7r : RWReg {
        value : 0,
       },
    bkp8r : RWReg {
        value : 0,
       },
    bkp9r : RWReg {
        value : 0,
       },
    bkp10r : RWReg {
        value : 0,
       },
    bkp11r : RWReg {
        value : 0,
       },
    bkp12r : RWReg {
        value : 0,
       },
    bkp13r : RWReg {
        value : 0,
       },
    bkp14r : RWReg {
        value : 0,
       },
    bkp15r : RWReg {
        value : 0,
       },
    bkp16r : RWReg {
        value : 0,
       },
    bkp17r : RWReg {
        value : 0,
       },
    bkp18r : RWReg {
        value : 0,
       },
    bkp19r : RWReg {
        value : 0,
       },
   },
  padding10: [0; 0x00D8],
  wwdg : WWDG_Type {
    cr : RWReg {
        value : 0,
       },
    cfr : RWReg {
        value : 0,
       },
    sr : RWReg {
        value : 0,
       },
   },
  padding11: [0; 0x00FD],
  iwdg : IWDG_Type {
    kr : RWReg {
        value : 0,
       },
    pr : RWReg {
        value : 0,
       },
    rlr : RWReg {
        value : 0,
       },
    sr : RWReg {
        value : 0,
       },
   },
  padding12: [0; 0x00FC],
  i2s2ext : SPI_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved3 : 0,
    crcpr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    rxcrcr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    txcrcr : RWReg {
        value : 0,
       },
    reserved6 : 0,
    i2scfgr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    i2spr : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding13: [0; 0x00F7],
  spi2 : SPI_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved3 : 0,
    crcpr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    rxcrcr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    txcrcr : RWReg {
        value : 0,
       },
    reserved6 : 0,
    i2scfgr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    i2spr : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding14: [0; 0x00F7],
  spi3 : SPI_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved3 : 0,
    crcpr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    rxcrcr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    txcrcr : RWReg {
        value : 0,
       },
    reserved6 : 0,
    i2scfgr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    i2spr : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding15: [0; 0x00F7],
  i2s3ext : SPI_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved3 : 0,
    crcpr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    rxcrcr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    txcrcr : RWReg {
        value : 0,
       },
    reserved6 : 0,
    i2scfgr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    i2spr : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding16: [0; 0x00F7],
  usart2 : USART_Type {
    sr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved1 : 0,
    brr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    cr1 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved4 : 0,
    cr3 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    gtpr : RWReg {
        value : 0,
       },
    reserved6 : 0,
   },
  padding17: [0; 0x00F9],
  usart3 : USART_Type {
    sr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved1 : 0,
    brr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    cr1 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved4 : 0,
    cr3 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    gtpr : RWReg {
        value : 0,
       },
    reserved6 : 0,
   },
  padding18: [0; 0x00F9],
  uart4 : USART_Type {
    sr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved1 : 0,
    brr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    cr1 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved4 : 0,
    cr3 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    gtpr : RWReg {
        value : 0,
       },
    reserved6 : 0,
   },
  padding19: [0; 0x00F9],
  uart5 : USART_Type {
    sr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved1 : 0,
    brr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    cr1 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved4 : 0,
    cr3 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    gtpr : RWReg {
        value : 0,
       },
    reserved6 : 0,
   },
  padding20: [0; 0x00F9],
  i2c1 : I2C_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    oar1 : RWReg {
        value : 0,
       },
    reserved2 : 0,
    oar2 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    sr1 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    sr2 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    trise : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding21: [0; 0x00F7],
  i2c2 : I2C_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    oar1 : RWReg {
        value : 0,
       },
    reserved2 : 0,
    oar2 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    sr1 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    sr2 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    trise : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding22: [0; 0x00F7],
  i2c3 : I2C_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    oar1 : RWReg {
        value : 0,
       },
    reserved2 : 0,
    oar2 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    sr1 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    sr2 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    trise : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding23: [0; 0x01F7],
  can1 : CAN_Type {
    mcr : RWReg {
        value : 0,
       },
    msr : RWReg {
        value : 0,
       },
    tsr : RWReg {
        value : 0,
       },
    rf0r : RWReg {
        value : 0,
       },
    rf1r : RWReg {
        value : 0,
       },
    ier : RWReg {
        value : 0,
       },
    esr : RWReg {
        value : 0,
       },
    btr : RWReg {
        value : 0,
       },
    reserved0 : [0; 0x0058],
    stxmailbox : [CAN_TxMailBox_Type {
        tir : RWReg {
            value : 0,
           },
        tdtr : RWReg {
            value : 0,
           },
        tdlr : RWReg {
            value : 0,
           },
        tdhr : RWReg {
            value : 0,
           },
       }; 0x0003],
    sfifomailbox : [CAN_FIFOMailBox_Type {
        rir : RWReg {
            value : 0,
           },
        rdtr : RWReg {
            value : 0,
           },
        rdlr : RWReg {
            value : 0,
           },
        rdhr : RWReg {
            value : 0,
           },
       }; 0x0002],
    reserved1 : [0; 0x000C],
    fmr : RWReg {
        value : 0,
       },
    fm1r : RWReg {
        value : 0,
       },
    reserved2 : 0,
    fs1r : RWReg {
        value : 0,
       },
    reserved3 : 0,
    ffa1r : RWReg {
        value : 0,
       },
    reserved4 : 0,
    fa1r : RWReg {
        value : 0,
       },
    reserved5 : [0; 0x0008],
    sfilterregister : [CAN_FilterRegister_Type {
        fr1 : RWReg {
            value : 0,
           },
        fr2 : RWReg {
            value : 0,
           },
       }; 0x001C],
   },
  padding24: [0; 0x0038],
  can2 : CAN_Type {
    mcr : RWReg {
        value : 0,
       },
    msr : RWReg {
        value : 0,
       },
    tsr : RWReg {
        value : 0,
       },
    rf0r : RWReg {
        value : 0,
       },
    rf1r : RWReg {
        value : 0,
       },
    ier : RWReg {
        value : 0,
       },
    esr : RWReg {
        value : 0,
       },
    btr : RWReg {
        value : 0,
       },
    reserved0 : [0; 0x0058],
    stxmailbox : [CAN_TxMailBox_Type {
        tir : RWReg {
            value : 0,
           },
        tdtr : RWReg {
            value : 0,
           },
        tdlr : RWReg {
            value : 0,
           },
        tdhr : RWReg {
            value : 0,
           },
       }; 0x0003],
    sfifomailbox : [CAN_FIFOMailBox_Type {
        rir : RWReg {
            value : 0,
           },
        rdtr : RWReg {
            value : 0,
           },
        rdlr : RWReg {
            value : 0,
           },
        rdhr : RWReg {
            value : 0,
           },
       }; 0x0002],
    reserved1 : [0; 0x000C],
    fmr : RWReg {
        value : 0,
       },
    fm1r : RWReg {
        value : 0,
       },
    reserved2 : 0,
    fs1r : RWReg {
        value : 0,
       },
    reserved3 : 0,
    ffa1r : RWReg {
        value : 0,
       },
    reserved4 : 0,
    fa1r : RWReg {
        value : 0,
       },
    reserved5 : [0; 0x0008],
    sfilterregister : [CAN_FilterRegister_Type {
        fr1 : RWReg {
            value : 0,
           },
        fr2 : RWReg {
            value : 0,
           },
       }; 0x001C],
   },
  padding25: [0; 0x0138],
  pwr : PWR_Type {
    cr : RWReg {
        value : 0,
       },
    csr : RWReg {
        value : 0,
       },
   },
  padding26: [0; 0x00FE],
  dac : DAC_Type {
    cr : RWReg {
        value : 0,
       },
    swtrigr : RWReg {
        value : 0,
       },
    dhr12r1 : RWReg {
        value : 0,
       },
    dhr12l1 : RWReg {
        value : 0,
       },
    dhr8r1 : RWReg {
        value : 0,
       },
    dhr12r2 : RWReg {
        value : 0,
       },
    dhr12l2 : RWReg {
        value : 0,
       },
    dhr8r2 : RWReg {
        value : 0,
       },
    dhr12rd : RWReg {
        value : 0,
       },
    dhr12ld : RWReg {
        value : 0,
       },
    dhr8rd : RWReg {
        value : 0,
       },
    dor1 : RWReg {
        value : 0,
       },
    dor2 : RWReg {
        value : 0,
       },
    sr : RWReg {
        value : 0,
       },
   },
  padding27: [0; 0x22F2],
  tim1 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding28: [0; 0x00EB],
  tim8 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding29: [0; 0x02EB],
  usart1 : USART_Type {
    sr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved1 : 0,
    brr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    cr1 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved4 : 0,
    cr3 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    gtpr : RWReg {
        value : 0,
       },
    reserved6 : 0,
   },
  padding30: [0; 0x00F9],
  usart6 : USART_Type {
    sr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved1 : 0,
    brr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    cr1 : RWReg {
        value : 0,
       },
    reserved3 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved4 : 0,
    cr3 : RWReg {
        value : 0,
       },
    reserved5 : 0,
    gtpr : RWReg {
        value : 0,
       },
    reserved6 : 0,
   },
  padding31: [0; 0x02F9],
  adc1 : ADC_Type {
    sr : RWReg {
        value : 0,
       },
    cr1 : RWReg {
        value : 0,
       },
    cr2 : RWReg {
        value : 0,
       },
    smpr1 : RWReg {
        value : 0,
       },
    smpr2 : RWReg {
        value : 0,
       },
    jofr1 : RWReg {
        value : 0,
       },
    jofr2 : RWReg {
        value : 0,
       },
    jofr3 : RWReg {
        value : 0,
       },
    jofr4 : RWReg {
        value : 0,
       },
    htr : RWReg {
        value : 0,
       },
    ltr : RWReg {
        value : 0,
       },
    sqr1 : RWReg {
        value : 0,
       },
    sqr2 : RWReg {
        value : 0,
       },
    sqr3 : RWReg {
        value : 0,
       },
    jsqr : RWReg {
        value : 0,
       },
    jdr1 : RWReg {
        value : 0,
       },
    jdr2 : RWReg {
        value : 0,
       },
    jdr3 : RWReg {
        value : 0,
       },
    jdr4 : RWReg {
        value : 0,
       },
    dr : RWReg {
        value : 0,
       },
   },
  padding32: [0; 0x002C],
  adc2 : ADC_Type {
    sr : RWReg {
        value : 0,
       },
    cr1 : RWReg {
        value : 0,
       },
    cr2 : RWReg {
        value : 0,
       },
    smpr1 : RWReg {
        value : 0,
       },
    smpr2 : RWReg {
        value : 0,
       },
    jofr1 : RWReg {
        value : 0,
       },
    jofr2 : RWReg {
        value : 0,
       },
    jofr3 : RWReg {
        value : 0,
       },
    jofr4 : RWReg {
        value : 0,
       },
    htr : RWReg {
        value : 0,
       },
    ltr : RWReg {
        value : 0,
       },
    sqr1 : RWReg {
        value : 0,
       },
    sqr2 : RWReg {
        value : 0,
       },
    sqr3 : RWReg {
        value : 0,
       },
    jsqr : RWReg {
        value : 0,
       },
    jdr1 : RWReg {
        value : 0,
       },
    jdr2 : RWReg {
        value : 0,
       },
    jdr3 : RWReg {
        value : 0,
       },
    jdr4 : RWReg {
        value : 0,
       },
    dr : RWReg {
        value : 0,
       },
   },
  padding33: [0; 0x002C],
  adc3 : ADC_Type {
    sr : RWReg {
        value : 0,
       },
    cr1 : RWReg {
        value : 0,
       },
    cr2 : RWReg {
        value : 0,
       },
    smpr1 : RWReg {
        value : 0,
       },
    smpr2 : RWReg {
        value : 0,
       },
    jofr1 : RWReg {
        value : 0,
       },
    jofr2 : RWReg {
        value : 0,
       },
    jofr3 : RWReg {
        value : 0,
       },
    jofr4 : RWReg {
        value : 0,
       },
    htr : RWReg {
        value : 0,
       },
    ltr : RWReg {
        value : 0,
       },
    sqr1 : RWReg {
        value : 0,
       },
    sqr2 : RWReg {
        value : 0,
       },
    sqr3 : RWReg {
        value : 0,
       },
    jsqr : RWReg {
        value : 0,
       },
    jdr1 : RWReg {
        value : 0,
       },
    jdr2 : RWReg {
        value : 0,
       },
    jdr3 : RWReg {
        value : 0,
       },
    jdr4 : RWReg {
        value : 0,
       },
    dr : RWReg {
        value : 0,
       },
   },
  padding34: [0; 0x002C],
  adc : ADC_Common_Type {
    csr : RWReg {
        value : 0,
       },
    ccr : RWReg {
        value : 0,
       },
    cdr : RWReg {
        value : 0,
       },
   },
  padding35: [0; 0x023D],
  sdio : SDIO_Type {
    power : RWReg {
        value : 0,
       },
    clkcr : RWReg {
        value : 0,
       },
    arg : RWReg {
        value : 0,
       },
    cmd : RWReg {
        value : 0,
       },
    respcmd : ROReg {
        value : 0,
       },
    resp1 : ROReg {
        value : 0,
       },
    resp2 : ROReg {
        value : 0,
       },
    resp3 : ROReg {
        value : 0,
       },
    resp4 : ROReg {
        value : 0,
       },
    dtimer : RWReg {
        value : 0,
       },
    dlen : RWReg {
        value : 0,
       },
    dctrl : RWReg {
        value : 0,
       },
    dcount : ROReg {
        value : 0,
       },
    sta : ROReg {
        value : 0,
       },
    icr : RWReg {
        value : 0,
       },
    mask : RWReg {
        value : 0,
       },
    reserved0 : [0; 0x0002],
    fifocnt : ROReg {
        value : 0,
       },
    reserved1 : [0; 0x000D],
    fifo : RWReg {
        value : 0,
       },
   },
  padding36: [0; 0x00DF],
  spi1 : SPI_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dr : RWReg {
        value : 0,
       },
    reserved3 : 0,
    crcpr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    rxcrcr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    txcrcr : RWReg {
        value : 0,
       },
    reserved6 : 0,
    i2scfgr : RWReg {
        value : 0,
       },
    reserved7 : 0,
    i2spr : RWReg {
        value : 0,
       },
    reserved8 : 0,
   },
  padding37: [0; 0x01F7],
  syscfg : SYSCFG_Type {
    memrmp : RWReg {
        value : 0,
       },
    pmc : RWReg {
        value : 0,
       },
    exticr : [RWReg {
        value : 0,
       }; 0x0004],
    reserved : [0; 0x0002],
    cmpcr : RWReg {
        value : 0,
       },
   },
  padding38: [0; 0x00F7],
  exti : EXTI_Type {
    imr : RWReg {
        value : 0,
       },
    emr : RWReg {
        value : 0,
       },
    rtsr : RWReg {
        value : 0,
       },
    ftsr : RWReg {
        value : 0,
       },
    swier : RWReg {
        value : 0,
       },
    pr : RWReg {
        value : 0,
       },
   },
  padding39: [0; 0x00FA],
  tim9 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding40: [0; 0x00EB],
  tim10 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding41: [0; 0x00EB],
  tim11 : TIM_Type {
    cr1 : RWReg {
        value : 0,
       },
    reserved0 : 0,
    cr2 : RWReg {
        value : 0,
       },
    reserved1 : 0,
    smcr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    dier : RWReg {
        value : 0,
       },
    reserved3 : 0,
    sr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    egr : RWReg {
        value : 0,
       },
    reserved5 : 0,
    ccmr1 : RWReg {
        value : 0,
       },
    reserved6 : 0,
    ccmr2 : RWReg {
        value : 0,
       },
    reserved7 : 0,
    ccer : RWReg {
        value : 0,
       },
    reserved8 : 0,
    cnt : RWReg {
        value : 0,
       },
    psc : RWReg {
        value : 0,
       },
    reserved9 : 0,
    arr : RWReg {
        value : 0,
       },
    rcr : RWReg {
        value : 0,
       },
    reserved10 : 0,
    ccr1 : RWReg {
        value : 0,
       },
    ccr2 : RWReg {
        value : 0,
       },
    ccr3 : RWReg {
        value : 0,
       },
    ccr4 : RWReg {
        value : 0,
       },
    bdtr : RWReg {
        value : 0,
       },
    reserved11 : 0,
    dcr : RWReg {
        value : 0,
       },
    reserved12 : 0,
    dmar : RWReg {
        value : 0,
       },
    reserved13 : 0,
    or : RWReg {
        value : 0,
       },
    reserved14 : 0,
   },
  padding42: [0; 0x2DEB],
  gpioa : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding43: [0; 0x00F6],
  gpiob : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding44: [0; 0x00F6],
  gpioc : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding45: [0; 0x00F6],
  gpiod : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding46: [0; 0x00F6],
  gpioe : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding47: [0; 0x00F6],
  gpiof : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding48: [0; 0x00F6],
  gpiog : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding49: [0; 0x00F6],
  gpioh : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding50: [0; 0x00F6],
  gpioi : GPIO_Type {
    moder : RWReg {
        value : 0,
       },
    otyper : RWReg {
        value : 0,
       },
    ospeedr : RWReg {
        value : 0,
       },
    pupdr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    odr : RWReg {
        value : 0,
       },
    bsrrl : RWReg {
        value : 0,
       },
    bsrrh : RWReg {
        value : 0,
       },
    lckr : RWReg {
        value : 0,
       },
    afr : [RWReg {
        value : 0,
       }; 0x0002],
   },
  padding51: [0; 0x03F6],
  crc : CRC_Type {
    dr : RWReg {
        value : 0,
       },
    idr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    reserved1 : 0,
    cr : RWReg {
        value : 0,
       },
   },
  padding52: [0; 0x01FD],
  rcc : RCC_Type {
    cr : RWReg {
        value : 0,
       },
    pllcfgr : RWReg {
        value : 0,
       },
    cfgr : RWReg {
        value : 0,
       },
    cir : RWReg {
        value : 0,
       },
    ahb1rstr : RWReg {
        value : 0,
       },
    ahb2rstr : RWReg {
        value : 0,
       },
    ahb3rstr : RWReg {
        value : 0,
       },
    reserved0 : 0,
    apb1rstr : RWReg {
        value : 0,
       },
    apb2rstr : RWReg {
        value : 0,
       },
    reserved1 : [0; 0x0002],
    ahb1enr : RWReg {
        value : 0,
       },
    ahb2enr : RWReg {
        value : 0,
       },
    ahb3enr : RWReg {
        value : 0,
       },
    reserved2 : 0,
    apb1enr : RWReg {
        value : 0,
       },
    apb2enr : RWReg {
        value : 0,
       },
    reserved3 : [0; 0x0002],
    ahb1lpenr : RWReg {
        value : 0,
       },
    ahb2lpenr : RWReg {
        value : 0,
       },
    ahb3lpenr : RWReg {
        value : 0,
       },
    reserved4 : 0,
    apb1lpenr : RWReg {
        value : 0,
       },
    apb2lpenr : RWReg {
        value : 0,
       },
    reserved5 : [0; 0x0002],
    bdcr : RWReg {
        value : 0,
       },
    csr : RWReg {
        value : 0,
       },
    reserved6 : [0; 0x0002],
    sscgr : RWReg {
        value : 0,
       },
    plli2scfgr : RWReg {
        value : 0,
       },
   },
  padding53: [0; 0x00DE],
  flash : FLASH_Type {
    acr : RWReg {
        value : 0,
       },
    keyr : RWReg {
        value : 0,
       },
    optkeyr : RWReg {
        value : 0,
       },
    sr : RWReg {
        value : 0,
       },
    cr : RWReg {
        value : 0,
       },
    optcr : RWReg {
        value : 0,
       },
   },
  padding54: [0; 0x08FA],
  dma1 : DMA_Type {
    lisr : RWReg {
        value : 0,
       },
    hisr : RWReg {
        value : 0,
       },
    lifcr : RWReg {
        value : 0,
       },
    hifcr : RWReg {
        value : 0,
       },
   },
  dma1_stream0 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream1 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream2 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream3 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream4 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream5 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream6 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma1_stream7 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  padding63: [0; 0x00CC],
  dma2 : DMA_Type {
    lisr : RWReg {
        value : 0,
       },
    hisr : RWReg {
        value : 0,
       },
    lifcr : RWReg {
        value : 0,
       },
    hifcr : RWReg {
        value : 0,
       },
   },
  dma2_stream0 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream1 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream2 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream3 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream4 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream5 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream6 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  dma2_stream7 : DMA_Stream_Type {
    cr : RWReg {
        value : 0,
       },
    ndtr : RWReg {
        value : 0,
       },
    par : RWReg {
        value : 0,
       },
    m0ar : RWReg {
        value : 0,
       },
    m1ar : RWReg {
        value : 0,
       },
    fcr : RWReg {
        value : 0,
       },
   },
  padding72: [0; 0x06CC],
  eth : ETH_Type {
    maccr : RWReg {
        value : 0,
       },
    macffr : RWReg {
        value : 0,
       },
    machthr : RWReg {
        value : 0,
       },
    machtlr : RWReg {
        value : 0,
       },
    macmiiar : RWReg {
        value : 0,
       },
    macmiidr : RWReg {
        value : 0,
       },
    macfcr : RWReg {
        value : 0,
       },
    macvlantr : RWReg {
        value : 0,
       },
    reserved0 : [0; 0x0002],
    macrwuffr : RWReg {
        value : 0,
       },
    macpmtcsr : RWReg {
        value : 0,
       },
    reserved1 : [0; 0x0002],
    macsr : RWReg {
        value : 0,
       },
    macimr : RWReg {
        value : 0,
       },
    maca0hr : RWReg {
        value : 0,
       },
    maca0lr : RWReg {
        value : 0,
       },
    maca1hr : RWReg {
        value : 0,
       },
    maca1lr : RWReg {
        value : 0,
       },
    maca2hr : RWReg {
        value : 0,
       },
    maca2lr : RWReg {
        value : 0,
       },
    maca3hr : RWReg {
        value : 0,
       },
    maca3lr : RWReg {
        value : 0,
       },
    reserved2 : [0; 0x0028],
    mmccr : RWReg {
        value : 0,
       },
    mmcrir : RWReg {
        value : 0,
       },
    mmctir : RWReg {
        value : 0,
       },
    mmcrimr : RWReg {
        value : 0,
       },
    mmctimr : RWReg {
        value : 0,
       },
    reserved3 : [0; 0x000E],
    mmctgfsccr : RWReg {
        value : 0,
       },
    mmctgfmsccr : RWReg {
        value : 0,
       },
    reserved4 : [0; 0x0005],
    mmctgfcr : RWReg {
        value : 0,
       },
    reserved5 : [0; 0x000A],
    mmcrfcecr : RWReg {
        value : 0,
       },
    mmcrfaecr : RWReg {
        value : 0,
       },
    reserved6 : [0; 0x000A],
    mmcrgufcr : RWReg {
        value : 0,
       },
    reserved7 : [0; 0x014E],
    ptptscr : RWReg {
        value : 0,
       },
    ptpssir : RWReg {
        value : 0,
       },
    ptptshr : RWReg {
        value : 0,
       },
    ptptslr : RWReg {
        value : 0,
       },
    ptptshur : RWReg {
        value : 0,
       },
    ptptslur : RWReg {
        value : 0,
       },
    ptptsar : RWReg {
        value : 0,
       },
    ptptthr : RWReg {
        value : 0,
       },
    ptpttlr : RWReg {
        value : 0,
       },
    reserved8 : RWReg {
        value : 0,
       },
    ptptssr : RWReg {
        value : 0,
       },
    reserved9 : [0; 0x0235],
    dmabmr : RWReg {
        value : 0,
       },
    dmatpdr : RWReg {
        value : 0,
       },
    dmarpdr : RWReg {
        value : 0,
       },
    dmardlar : RWReg {
        value : 0,
       },
    dmatdlar : RWReg {
        value : 0,
       },
    dmasr : RWReg {
        value : 0,
       },
    dmaomr : RWReg {
        value : 0,
       },
    dmaier : RWReg {
        value : 0,
       },
    dmamfbocr : RWReg {
        value : 0,
       },
    dmarswtr : RWReg {
        value : 0,
       },
    reserved10 : [0; 0x0008],
    dmachtdr : RWReg {
        value : 0,
       },
    dmachrdr : RWReg {
        value : 0,
       },
    dmachtbar : RWReg {
        value : 0,
       },
    dmachrbar : RWReg {
        value : 0,
       },
   },
};
