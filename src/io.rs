#![allow(dead_code)]
use stm::{STM, GPIO_Type};
use system::clk_en_ahb1;
/**
    Pro cviky nad perifernímy registry je možné použít dva přístupy:
  1. Stejně jako v C použít raw pointer na periferní skupinu a pak
     používat volatile_load, store - to vyžaduje dereference toho
     ukazatele, což se musí umístit do unsafe bloku.
  2. Zde jsem zkusil něco trochu jiného. Vytvoříme celou kompaktní strukturu
     pro velkou část periferních registrů (celé by to bylo obrovské) a umístíme
     to do paměti linker skriptem jako statickou strukturu. Je to dost podivné,
     strukturu (STM) překladač vlastně vidí jako konstantní, ale do položek je možné
     pomocí volatile_store normálně zapisovat. Dále je nutné strukturu inicializovat,
     což je dost divoké (je to generováno z AST původních C zdrojáků) a nakonec
     se to stejně nepoužije (sekce je NOLOAD). Ale překladač nenadává. Možná až
     pochopím princip fungování toho jazyka, zjistím, že je to celé blbost.
     Ten rust nějak moc nepočítá se statickými proměnnými (z hlediska vyšší úrovně
     programování není divu), což je u vestavných systému (bez OS) dost neobvyklé.
*/
#[derive(Copy, Clone)]
pub enum PortNo {
  PortA=0,PortB,PortC,PortD,PortE,
}
#[derive(Copy, Clone)]
pub enum PinMode {
  ModeIn=0, ModeOut, ModeAF, ModeAnalog,
}
// Místo tabulek - pro statické odkazy nejde tabulka použít.
// Kód sice naroste, ale není to dramatické. Na druhou stranu
// není nutné používat dereference raw pointerů v unsafe bloku.
fn get_clk (io:PortNo) -> u32 {
  match io {
    PortNo::PortA => return 0x01u32,
    PortNo::PortB => return 0x02u32,
    PortNo::PortC => return 0x04u32,
    PortNo::PortD => return 0x08u32,
    PortNo::PortE => return 0x10u32,
  }
}
fn get_port (io:PortNo) -> &'static GPIO_Type {
  match io {
    PortNo::PortA => return &STM.gpioa,
    PortNo::PortB => return &STM.gpiob,
    PortNo::PortC => return &STM.gpioc,
    PortNo::PortD => return &STM.gpiod,
    PortNo::PortE => return &STM.gpioe,
  }
}
/************************ Podobně jako C++ třída **********************/
pub struct Pin {        // Sama struktura drží co nejméně (neměnných) položek
  position: u16,                // pozice pinu na portu
  io      : PortNo,             // číslo portu  (jen init, ale musí být)
  port    : &'static GPIO_Type, // port zde, pokud je konstruktor konstantní, nejde to
}
impl Pin {
  // "Konstruktor"
  pub fn new (position:u16, io:PortNo) -> Pin {
    Pin {position:position, io:io, port:get_port(io)}
  }
  // Inicializace do určeného módu
  pub fn init (&self, mode:PinMode) {
    clk_en_ahb1 (get_clk (self.io));
    let mpos: u16 = 2u16 * self.position;

    self.port.moder.modify(|r| {
      let mut om = r;
      om &= !(3u32         << mpos);
      om |=  (mode as u32) << mpos;
      return om;
    });
  }
  // Změna stavu
  pub fn toggle (&self) {
    self.port.odr.modify (|r| r ^ (1u32 << self.position));
  }
  // Nastavení na bool hodnotu
  pub fn set (&self, val:bool) {
    if val {self.port.bsrrl.set (1u16 << self.position)}
    else   {self.port.bsrrh.set (1u16 << self.position)}
  }
  // Přečtení hodnoty (pro vstupní pin)
  pub fn get (&self) -> bool {
    if (self.port.idr.get() & (1u32 << self.position)) != 0u32 {true} else {false}
  }
}
/***********************************************************************/
// Ojebávka pro interrupt - aby to chodilo jako v C. Proč - zatím nevím.
unsafe impl Sync for Pin {}

