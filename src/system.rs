//#![allow(dead_code)]
use stm::{STM, SCB, NVIC, SYSTICK, DBGMCU};

#[inline(always)]
pub fn clk_en_ahb1 (pos: u32) {
  STM.rcc.ahb1enr.modify (|r| r | pos);
}
extern {
  static _vect_tab_begin: u32;
}
/* UNUSED CONSTS
const AHBPRESC_TABLE: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 4, 6, 7, 8, 9];
const RCC_CFGR_HPRE_DIV2: u32 = 0x00000080u32;        // SYSCLK divided by 2
const RCC_CFGR_HPRE_DIV4: u32 = 0x00000090u32;        // SYSCLK divided by 4
const RCC_CFGR_SW_HSI   : u32 = 0x00000000u32;        // HSI selected as system clock
const RCC_CFGR_SW_HSE   : u32 = 0x00000001u32;        // HSE selected as system clock
const RCC_CFGR_SWS_HSI  : u32 = 0x00000000u32;        // HSI oscillator used as system clock
const RCC_CFGR_SWS_HSE  : u32 = 0x00000004u32;        // HSE oscillator used as system clock
const RCC_PLLCFGR_PLLSRC_HSI:u32 = 0x00000000u32;
*/

const HSE_STARTUP_TIMEOUT:u32 = 0x00000500u32;        /* Time out for HSE start up */
const RCC_APB1ENR_PWREN:  u32 = 0x10000000u32;
const PWR_CR_PMODE:       u32 = 0x00004000u32;
const RCC_CFGR_HPRE_DIV1: u32 = 0x00000000u32;        /* SYSCLK not divided */
const RCC_CFGR_PPRE2_DIV2:u32 = 0x00008000u32;        /* HCLK divided by 2 */
const RCC_CFGR_PPRE1_DIV4:u32 = 0x00001400u32;        /* HCLK divided by 4 */

const RCC_CR_HSEON  :  u32 = 0x00010000u32;
const RCC_CR_HSERDY :  u32 = 0x00020000u32;
const RCC_CR_PLLON:    u32 = 0x01000000u32;
const RCC_CR_PLLRDY:   u32 = 0x02000000u32;
const RCC_CR_PLLI2SON: u32 = 0x04000000u32;
const RCC_CR_PLLI2SRDY:u32 = 0x08000000u32;

const RCC_CFGR_I2SSRC: u32 = 0x00800000u32;
const RCC_CFGR_SW:     u32 = 0x00000003u32;        /* SW[1:0] bits (System clock Switch) */
const RCC_CFGR_SW_PLL: u32 = 0x00000002u32;        /* PLL selected as system clock */
const RCC_CFGR_SWS:    u32 = 0x0000000Cu32;        /* SWS[1:0] bits (System Clock Switch Status) */
const RCC_CFGR_SWS_PLL:u32 = 0x00000008u32;        /* PLL used as system clock */

const FLASH_ACR_ICEN:  u32 = 0x00000200u32;
const FLASH_ACR_DCEN:  u32 = 0x00000400u32;
const RCC_PLLCFGR_PLLSRC_HSE:u32 = 0x00400000u32;
const FLASH_ACR_LATENCY_5WS: u32 = 0x00000005u32;

/* PLL_VCO = (HSE_VALUE or HSI_VALUE / PLL_M) * PLL_N */
const PLL_M:u32    =    8u32;
const PLL_N:u32    =    336u32;
/* SYSCLK = PLL_VCO / PLL_P */
const PLL_P:u32    =    2u32;
/* USB OTG FS, SDIO and RNG Clock =  PLL_VCO / PLLQ */
const PLL_Q:u32    =    7u32;
/* PLLI2S_VCO = (HSE_VALUE Or HSI_VALUE / PLL_M) * PLLI2S_N
   I2SCLK = PLLI2S_VCO / PLLI2S_R */
const PLLI2S_N:u32 =    192u32;
const PLLI2S_R:u32 =    5u32;

#[inline(always)]
fn set_sys_clock () {
  /******************************************************************************/
  /*            PLL (clocked by HSE) used as System clock source                */
  /******************************************************************************/
  let mut startupcounter:u32 = 0u32;

  /* Enable HSE */
  STM.rcc.cr.modify(|r| r | RCC_CR_HSEON);
  /* Wait till HSE is ready and if Time out is reached exit */
  let mut hsestatus = STM.rcc.cr.get() & RCC_CR_HSERDY;
  while (hsestatus == 0) && (startupcounter != HSE_STARTUP_TIMEOUT) {
    hsestatus = STM.rcc.cr.get() & RCC_CR_HSERDY;
    startupcounter += 1u32;
  }
  if (STM.rcc.cr.get() & RCC_CR_HSERDY) != 0u32 {
    hsestatus = 0x01u32;
  } else {
    hsestatus = 0x00u32;
  }
  if hsestatus == 0x01u32 {
    /* Enable high performance mode, System frequency up to 168 MHz */
    STM.rcc.apb1enr.set(STM.rcc.apb1enr.get() | RCC_APB1ENR_PWREN);
    STM.rcc.cr.set(STM.rcc.cr.get() | PWR_CR_PMODE);
    /* HCLK = SYSCLK / 1 */
    STM.rcc.cfgr.modify (|r| r | RCC_CFGR_HPRE_DIV1);
    /* PCLK2 = HCLK / 2 */
    STM.rcc.cfgr.modify (|r| r | RCC_CFGR_PPRE2_DIV2);
    /* PCLK1 = HCLK / 4 */
    STM.rcc.cfgr.modify (|r| r | RCC_CFGR_PPRE1_DIV4);
    /* Configure the main PLL */
    STM.rcc.pllcfgr.set(PLL_M | (PLL_N << 6) | (( (PLL_P >> 1) - 1) << 16) |
                       RCC_PLLCFGR_PLLSRC_HSE |  (PLL_Q << 24));
    /* Enable the main PLL */
    STM.rcc.cr.modify (|r| r | RCC_CR_PLLON);
    /* Wait till the main PLL is ready */
    while (STM.rcc.cr.get() & RCC_CR_PLLRDY) == 0 {}
    /* Configure Flash prefetch, Instruction cache, Data cache and wait state */
    STM.flash.acr.set(FLASH_ACR_ICEN | FLASH_ACR_DCEN | FLASH_ACR_LATENCY_5WS);
    /* Select the main PLL as system clock source */
    let mut tcfg = STM.rcc.cfgr.get();
    tcfg &= !RCC_CFGR_SW;
    tcfg |=  RCC_CFGR_SW_PLL;
    STM.rcc.cfgr.set(tcfg);
    /* Wait till the main PLL is used as system clock source */
    while (STM.rcc.cfgr.get() & RCC_CFGR_SWS) != RCC_CFGR_SWS_PLL {};
  }
  /******************************************************************************/
  /*                        I2S clock configuration                             */
  /******************************************************************************/
  /* PLLI2S clock used as I2S clock source */
  STM.rcc.cfgr.modify (|r| r & !RCC_CFGR_I2SSRC);
  /* Configure PLLI2S */
  STM.rcc.plli2scfgr.set((PLLI2S_N << 6) | (PLLI2S_R << 28));
  /* Enable PLLI2S */
  STM.rcc.cr.modify (|r| r | RCC_CR_PLLI2SON);
  /* Wait till PLLI2S is ready */
  while (STM.rcc.cr.get() & RCC_CR_PLLI2SRDY) == 0 {};
}

pub fn system_init () {
  unsafe {
    (*SCB).cpacr.modify (|r| r | (3u32 << 10*2) | (3u32 << 11*2));  /* set CP10 and CP11 Full Access */
    /* Reset the RCC clock configuration to the default reset state ------------*/
  }
  /* Set HSION bit */
  STM.rcc.cr.set(STM.rcc.cr.get() | 0x00000001u32);
  /* Reset CFGR register */
  STM.rcc.cfgr.set(0x00000000u32);
  /* Reset HSEON, CSSON and PLLON bits */
  STM.rcc.cr.modify (|r| r & 0xFEF6FFFFu32);
  /* Reset PLLCFGR register */
  STM.rcc.pllcfgr.set(0x24003010u32);
  /* Reset HSEBYP bit */
  STM.rcc.cr.modify (|r| r & 0xFFFBFFFFu32);
  /* Disable all interrupts */
  STM.rcc.cir.set(0x00000000u32);
  /* Configure the System clock source, PLL Multiplier and Divider factors,
     AHB/APBx prescalers and Flash settings ----------------------------------*/
  set_sys_clock();
  unsafe {
    /* Configure the Vector Table location add offset address ------------------*/
    let tmp : *const u32 = &_vect_tab_begin;
    (*SCB).vtor.set (tmp as u32); /* Vector Table Relocation in SRAM/FLASH */
  }
}
const HSE_VALUE:u32 =  8000000u32; /* Value of the External oscillator in Hz */
const HSI_VALUE:u32 = 16000000u32; /* Value of the Internal oscillator in Hz*/

const RCC_PLLCFGR_PLLM:  u32 = 0x0000003Fu32;
const RCC_PLLCFGR_PLLN:  u32 = 0x00007FC0u32;
const RCC_PLLCFGR_PLLP:  u32 = 0x00030000u32;
const RCC_PLLCFGR_PLLSRC:u32 = 0x00400000u32;

pub static mut SYSTEM_CORE_CLOCK: u32 = 168000000u32;

#[inline(always)]
fn clock_pll() -> u32 {
  let mut result = 0u32;
  let pllsource  = (STM.rcc.pllcfgr.get() & RCC_PLLCFGR_PLLSRC) >> 22;
  let pllm       =  STM.rcc.pllcfgr.get() & RCC_PLLCFGR_PLLM;

  let mut pllvco = (STM.rcc.pllcfgr.get() & RCC_PLLCFGR_PLLN) >> 6;
  if pllm != 0u32 {       // prevence chyb pri deleni 0
    if pllsource != 0 {
      pllvco *= HSE_VALUE / pllm;
    } else {
      pllvco *= HSI_VALUE / pllm;
    };
  } else {
    return result;
  };

  let pllp = (((STM.rcc.pllcfgr.get() & RCC_PLLCFGR_PLLP) >> 16) + 1u32) * 2u32;
  if pllp != 0u32 {
    result = pllvco / pllp;
  } else {
    return result;
  };
  return result;
}

pub fn system_core_clock_update () {
  unsafe {  // SYSTEM_CORE_CLOCK
    /* Get SYSCLK source -------------------------------------------------------*/
    let tmp = STM.rcc.cfgr.get() & RCC_CFGR_SWS;
    match tmp {
      0x00u32 => SYSTEM_CORE_CLOCK = HSI_VALUE,
      0x04u32 => SYSTEM_CORE_CLOCK = HSE_VALUE,
      0x08u32 => SYSTEM_CORE_CLOCK = clock_pll(),
      _       => SYSTEM_CORE_CLOCK = HSI_VALUE,
    }
  }
}
pub enum IRQn {
/******  Cortex-M4 Processor Exceptions Numbers ****************************************************************/
  NonMaskableIntIRQn         = -14,    /* 2 Non Maskable Interrupt                                          */
  MemoryManagementIRQn       = -12,    /* 4 Cortex-M4 Memory Management Interrupt                           */
  BusFaultIRQn               = -11,    /* 5 Cortex-M4 Bus Fault Interrupt                                   */
  UsageFaultIRQn             = -10,    /* 6 Cortex-M4 Usage Fault Interrupt                                 */
  SVCallIRQn                 = -5,     /* 11 Cortex-M4 SV Call Interrupt                                    */
  DebugMonitorIRQn           = -4,     /* 12 Cortex-M4 Debug Monitor Interrupt                              */
  PendSVIRQn                 = -2,     /* 14 Cortex-M4 Pend SV Interrupt                                    */
  SysTickIRQn                = -1,     /* 15 Cortex-M4 System Tick Interrupt                                */
  // TODO
}

const __NVIC_PRIO_BITS:u8 =  4u8;

pub fn nvic_set_priority(irqn:IRQn, priority:u8) {
  let ti: i16 = irqn as i16;
  unsafe {
    if ti < 0 {
      let index:u32 = ((ti as u32) & 0x0Fu32) - 4u32;
      (*SCB).shp[index as usize].set((priority << (8u8 - __NVIC_PRIO_BITS)) & 0xffu8);
    } else {
      (*NVIC).ip[ti    as usize].set((priority << (8u8 - __NVIC_PRIO_BITS)) & 0xffu8);
    };
  }
}
pub fn nvic_enableirq(irqn:IRQn) {
  let index = irqn as u32;
  unsafe {
    (*NVIC).iser[(index >> 5) as usize].set(1u32 << (index & 0x1Fu32)); /* enable interrupt */
  }
}

const SYSTICK_LOAD_RELOAD_MSK:   u32 = 0x00FF_FFFFu32;      /* SysTick LOAD: RELOAD Mask    */
const SYSTICK_CTRL_CLKSOURCE_MSK:u32 = 1u32 << 2;           /* SysTick CTRL: CLKSOURCE Mask */
const SYSTICK_CTRL_TICKINT_MSK  :u32 = 1u32 << 1;           /* SysTick CTRL: TICKINT Mask   */
const SYSTICK_CTRL_ENABLE_MSK   :u32 = 1u32;                /* SysTick CTRL: ENABLE Mask    */

pub fn systick_config(ticks:u32) {
  if ticks > SYSTICK_LOAD_RELOAD_MSK {return;};             /* Reload value impossible */
  unsafe {
    (*SYSTICK).load.set((ticks & SYSTICK_LOAD_RELOAD_MSK) - 1u32);      /* set reload register */
    nvic_set_priority (IRQn::SysTickIRQn, (1<<__NVIC_PRIO_BITS) - 1);   /* set Priority for Cortex-M0 System Interrupts */
    (*SYSTICK).val.set(0);                                              /* Load the SysTick Counter Value */
    (*SYSTICK).ctrl.set(SYSTICK_CTRL_CLKSOURCE_MSK |
                     SYSTICK_CTRL_TICKINT_MSK      |
                     SYSTICK_CTRL_ENABLE_MSK);                          /* Enable SysTick IRQ and SysTick Timer */
  }
}
/// Pokud použijeme v programu wfi/wfe, je nutné povolit debug i v tt. módu.
pub fn enable_debug_on_sleep () {
  unsafe {
    (*DBGMCU).cr.set (1u32 << 0);
  }
}

