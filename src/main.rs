#![feature(lang_items,start,asm,core_intrinsics)]
#![no_std]
#![no_main]

/*  Příklad je vlastně přepsán z C, což je pro rust dost divočina, ale
    není nutné používat různé divné crate, které jsou pro mne jako začátečníka
    v tomto jazyku dost nepochopitelné. A protože jsem zvyklý spíš na C++,
    učinil jsem pokus trochu kód zapouzdřit, což je spíš pro budoucí aplikace,
    zde je to dost zbytečné a ani to nepřispívá k přehlednosti.

    Snad se podařilo vyřešit všechny problémy, které by se snad mohly vyskytnout
    při psaní ovladačů periferií. Ten rust je _hodně_ omezující. Především vytváří
    proměnné default na zásobníku, což je normální, a lze to tak dělat i v C a C++,
    ale ukázalo se, že v přerušení statické proměnné zřejmě potřeba budou a to
    je problém. Na webu se dají najít příklady kódu - žádný ale přerušení nepoužívá,
    i když se tváří, že je to vyřešeno, pěkně a bezpečně podle všech nových poznatků
    počítačové vědy. Má to jedinou chybu - nejde tak napsat funkční kód, který by
    něco rozumného dělal.
*/
pub mod base;     // cca startup.c
pub mod system;   // cca system_stm32XX.c
pub mod stm;      // cca stm32XX.h (bez definic jednotlivých bitů)
pub mod io;       // gpio zhruba jako třída v C++
pub mod vc;       // rust specificky - volatile load / store

/******************************************************************************/
fn wait () {
  unsafe {
    asm!("wfi" :::: "volatile");
  }
}
use base::  { init_data };
use io::    { Pin, PortNo, PinMode };
use system::{ system_init, system_core_clock_update, systick_config,
              SYSTEM_CORE_CLOCK, enable_debug_on_sleep };
/******************************************************************************/
struct Test {                   // zapouzdření testu
  counter: u32,
  red:     Pin,
}
impl Test {                     // a jeho primitivní funkce
  fn new () -> Test {           // "konstruktor"
    Test {counter:0, red: Pin::new(14u16, PortNo::PortD)}
  }
  fn init (&self) {             // inicializace, včetně systick
    self.red.init(PinMode::ModeOut);
    // nutnost unsafe zde nechápu (jen čtení ze SYSTEM_CORE_CLOCK)
    let clk; unsafe { clk = SYSTEM_CORE_CLOCK / 1000; }
    systick_config (clk);       // nastaví periodu + zapne přerušení
  }
  // Testovací metoda. Také jen blikání, čítač vyžaduje mutabilitu
  fn pass (&mut self) {         // volano v přerušení po 1 ms
    self.counter += 1;
    if self.counter >= 500 {
      self.counter = 0;
      self.red.toggle();        // perioda 1 s
    }
  }
}
/******************************************************************************
 Rust si se statickými proměnnými moc netyká, jde to trochu obejít, existuje
 ještě crate lazy_static, které umožní používat statické proměnné poměrně
 bezpečně, ale na ARM nefunguje a je to dělo na komára (v rustu ostatně vše) */
static mut ST: Option<Test> = None;  // Option umožní inicializovat až v těle main
/******************************************************************************/
// Application main()
// Musí být také bez návratu
fn main () -> ! {
  unsafe {
    ST = Some (Test::new());            // inicializace statické struktury až zde
    ST.as_mut().unwrap().init();        // zde překladač vidí, že Option je Some
  }
  let blue = Pin::new (15u16, PortNo::PortD);         // zde jsou ty Pin_y
  let oran = Pin::new (13u16, PortNo::PortD);         // vlastně taky konstantní
  blue.init(PinMode::ModeOut);
  oran.init(PinMode::ModeOut);
  oran.set (true);                                    // jen rozsvitit
  loop {
    blue.toggle();                                    // perioda 2 ms
    wait();
  }
}
/// This function will be "called" by the processor at reset.  Note that none of
/// the C or Rust environment has been established --- in particular, this
/// function is responsible for initializing any global data it might need!
/// I've currently punted on this for simplicity.
pub extern fn reset_handler() -> ! {
  init_data();
  system_init();
  enable_debug_on_sleep();
  system_core_clock_update();                         // systém nastaven
  // Uživatelský program - stejně jako v C
  main();
}
// Funkce je unsafe kvůli statickým proměnným v jejím těle.
pub unsafe extern fn systick_handler() {              // přerušení po 1 ms
  /* Použití Option umožní přístup ke statickým datům. Není to bez problémů.
     1. Je to divné.
     2. Není to bezpečné.
     3. Nafoukne to kód (panic - unwrap, chtělo to match).
        ST.as_mut().unwrap().pass(); // takhle tedy ne

     Ale funguje to. V přerušovací rutině asi nějaký odkaz na statická data
     být musí. Alespoň zatím nevím jak to udělat bez něj. Prostě uvnitř téhle
     funkce musí být něco, co odkazuje někam "ven". Dostat to tam jinak než
     jako statická data neumím. A raw pointer v rustu nějak nefunguje. V podstatě
     je podobné jako zápis v C++

     struct Test;             // forward decl. pro následující ukazatel
     static Test * pTest = 0; // ukazatel na instanci - přes něj pak voláme
     struct Test {
       Test() {               // v konstruktoru přiřadím ukazateli hodnotu
         pTest = this;
       };
       void pass () {};       // nějaká metoda ...
     };
     // statický konstruktor - v C++ je volán z jiné části než main, není podstatné
     // pokud použiju volání přes ukazatel, instance musí být jediná,
     static Test t;               //  u periferie to tak vždy bývá

     void Handler () {
       if (pTest) pTest->pass();  // takhle v C++, je to volnější, instanci nemusím mít předem
       t.pass();                  // takhle v rustu, omezuje na již vytvořenou instanci,
     }                            // ale zase tak moc restriktivní to není aby se to nedalo použít
     // ...
  */
  match ST.as_mut() {             // takhle je to lepší
    Some(n) => n.pass(),
    None => {},                   // not panic
  }
}

