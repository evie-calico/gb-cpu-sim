use try_from_discrim::TryFrom;

#[derive(Debug, TryFrom)]
#[from(u16)]
#[non_exhaustive]
/// A collection of hardware registers' addresses, extracted from [`hardware.inc`](https://github.com/gbdev/hardware.inc).
pub enum HwReg {
	/// MBC SRAM enable.
	Ramg = 0x0000,
	/// MBC ROM bank switch, low 8 bits.
	Romb0 = 0x2000,
	/// MBC ROM bank switch, upper 8 bits.
	Romb1 = 0x3000,
	/// MBC SRAM bank switch.
	Ramb = 0x4000,
	/// MBC RTC latch toggle.
	Rtclatch = 0x6000,

	/// Joypad.
	P1 = 0xFF00,

	/// Serial data.
	Sb = 0xFF01,
	/// Serial control.
	Sc = 0xFF02,

	/// Divided clock counter.
	Div = 0xFF04,
	/// Timer counter.
	Tima = 0xFF05,
	/// Timer modulo.
	Tma = 0xFF06,
	/// Timer control.
	Tac = 0xFF07,

	/// Pending interrupts.
	If = 0xFF0F,

	/// CH1 frequency sweep.
	Nr10 = 0xFF10,
	/// CH1 duty control & sound length.
	Nr11 = 0xFF11,
	/// CH1 volume control.
	Nr12 = 0xFF12,
	/// CH1 wave_length, low 8 bits.
	Nr13 = 0xFF13,
	/// CH1 wave_length, upper 3 bits & control.
	Nr14 = 0xFF14,
	/// CH2 duty control & sound length.
	Nr21 = 0xFF16,
	/// CH2 volume control.
	Nr22 = 0xFF17,
	/// CH2 wave_length, low 8 bits.
	Nr23 = 0xFF18,
	/// CH2 wave_length, upper 3 bits & control.
	Nr24 = 0xFF19,
	/// CH3 enable.
	Nr30 = 0xFF1A,
	/// CH3 sound length.
	Nr31 = 0xFF1B,
	/// CH3 volume control.
	Nr32 = 0xFF1C,
	/// CH3 wave_length, low 8 bits.
	Nr33 = 0xFF1D,
	/// CH3 wave_length, upper 3 bits.
	Nr34 = 0xFF1E,
	/// CH4 sound length.
	Nr41 = 0xFF20,
	/// CH4 volume control.
	Nr42 = 0xFF21,
	/// CH4 LFSR control.
	Nr43 = 0xFF22,
	/// CH4 control.
	Nr44 = 0xFF23,
	/// Master volume & VIN panning.
	Nr50 = 0xFF24,
	/// Sound panning.
	Nr51 = 0xFF25,
	/// Audio control.
	Nr52 = 0xFF26,

	Wave0 = 0xFF30,
	Wave1 = 0xFF31,
	Wave2 = 0xFF32,
	Wave3 = 0xFF33,
	Wave4 = 0xFF34,
	Wave5 = 0xFF35,
	Wave6 = 0xFF36,
	Wave7 = 0xFF37,
	Wave8 = 0xFF38,
	Wave9 = 0xFF39,
	WaveA = 0xFF3A,
	WaveB = 0xFF3B,
	WaveC = 0xFF3C,
	WaveD = 0xFF3D,
	WaveE = 0xFF3E,
	WaveF = 0xFF3F,

	/// LCD control.
	Lcdc = 0xFF40,
	/// LCD status.
	Stat = 0xFF41,
	/// Viewport vertical offset.
	Scy = 0xFF42,
	/// Viewport horizontal offset.
	Scx = 0xFF43,
	/// Current scanline.
	Ly = 0xFF44,
	/// LY comparison.
	Lyc = 0xFF45,
	/// OAM DMA source & start.
	Dma = 0xFF46,
	/// DMG background palette.
	Bgp = 0xFF47,
	/// DMG OBJ palette 0.
	Obp0 = 0xFF48,
	/// DMG OBJ palette 1.
	Obp1 = 0xFF49,
	/// Window Y coordinate.
	Wy = 0xFF4A,
	/// Window X coordinate.
	Wx = 0xFF4B,

	/// CGB speed switch.
	Key1 = 0xFF4D,

	/// CGB VRAM bank switch.
	Vbk = 0xFF4F,

	/// CGB DMA source, upper 8 bits.
	Hdma1 = 0xFF51,
	/// CGB DMA source, lower 8 bits.
	Hdma2 = 0xFF52,
	/// CGB DMA destination, upper 8 bits.
	Hdma3 = 0xFF53,
	/// CGB DMA destination, lower 8 bits.
	Hdma4 = 0xFF54,
	/// CGB DMA length & mode & start.
	Hdma5 = 0xFF55,

	/// CGB IR.
	Rp = 0xFF56,

	/// CGB BG palette address.
	Bcps = 0xFF68,
	/// CGB BG palette data.
	Bcpd = 0xFF69,
	/// CGB OBJ palette address.
	Ocps = 0xFF6A,
	/// CGB OBJ palette data.
	Ocpd = 0xFF6B,

	/// CGB WRAM bank switch.
	Svbk = 0xFF70,

	/// CH1 & CH2 digital output.
	Pcm12 = 0xFF76,
	/// CH3 & CH4 digital output.
	Pcm34 = 0xFF77,

	/// Enabled interrupts.
	Ie = 0xFFFF,
}

impl From<HwReg> for u16 {
	fn from(reg: HwReg) -> Self {
		reg as u16
	}
}

#[derive(Default)]
pub struct Mmio {
	ramg: u8,
	romb0: u8,
	romb1: u8,
	ramb: u8,
	rtclatch: u8,
	p1: u8,
	sb: u8,
	sc: u8,
	div: u8,
	tima: u8,
	tma: u8,
	tac: u8,
	_if: u8,
	// Sound registers
	nr10: u8,
	nr11: u8,
	nr12: u8,
	nr13: u8,
	nr14: u8,
	nr21: u8,
	nr22: u8,
	nr23: u8,
	nr24: u8,
	nr30: u8,
	nr31: u8,
	nr32: u8,
	nr33: u8,
	nr34: u8,
	nr41: u8,
	nr42: u8,
	nr43: u8,
	nr44: u8,
	nr50: u8,
	nr51: u8,
	nr52: u8,
	wave_0: u8,
	wave_1: u8,
	wave_2: u8,
	wave_3: u8,
	wave_4: u8,
	wave_5: u8,
	wave_6: u8,
	wave_7: u8,
	wave_8: u8,
	wave_9: u8,
	wave_a: u8,
	wave_b: u8,
	wave_c: u8,
	wave_d: u8,
	wave_e: u8,
	wave_f: u8,
	lcdc: u8,
	stat: u8,
	scy: u8,
	scx: u8,
	ly: u8,
	lyc: u8,
	dma: u8,
	bgp: u8,
	obp0: u8,
	obp1: u8,
	wy: u8,
	wx: u8,
	key1: u8,
	vbk: u8,
	hdma1: u8,
	hdma2: u8,
	hdma3: u8,
	hdma4: u8,
	hdma5: u8,
	rp: u8,
	bcps: u8,
	bcpd: u8,
	ocps: u8,
	ocpd: u8,
	svbk: u8,
	pcm12: u8,
	pcm34: u8,
	ie: u8,
}

impl Mmio {
	pub fn read(&self, register: HwReg) -> u8 {
		match register {
			HwReg::Ramg => self.ramg,
			HwReg::Romb0 => self.romb0,
			HwReg::Romb1 => self.romb1,
			HwReg::Ramb => self.ramb,
			HwReg::Rtclatch => self.rtclatch,
			HwReg::P1 => {
				// TODO: Allow the user to define a set of pressed buttons to loosely implement rP1.
				eprintln!("Unit test has no joypad input");
				self.p1
			}
			HwReg::Sb => {
				// TODO: Allow the user to define a stream of bytes to read through rSB.
				eprintln!("Unit test has no serial input");
				self.sb
			}
			HwReg::Sc => self.sc,
			HwReg::Div => self.div,
			HwReg::Tima => self.tima,
			HwReg::Tma => self.tma,
			HwReg::Tac => self.tac,
			HwReg::If => self._if,
			HwReg::Nr10 => self.nr10,
			HwReg::Nr11 => self.nr11,
			HwReg::Nr12 => self.nr12,
			HwReg::Nr13 => self.nr13,
			HwReg::Nr14 => self.nr14,
			HwReg::Nr21 => self.nr21,
			HwReg::Nr22 => self.nr22,
			HwReg::Nr23 => self.nr23,
			HwReg::Nr24 => self.nr24,
			HwReg::Nr30 => self.nr30,
			HwReg::Nr31 => self.nr31,
			HwReg::Nr32 => self.nr32,
			HwReg::Nr33 => self.nr33,
			HwReg::Nr34 => self.nr34,
			HwReg::Nr41 => self.nr41,
			HwReg::Nr42 => self.nr42,
			HwReg::Nr43 => self.nr43,
			HwReg::Nr44 => self.nr44,
			HwReg::Nr50 => self.nr50,
			HwReg::Nr51 => self.nr51,
			HwReg::Nr52 => self.nr52,
			HwReg::Wave0 => self.wave_0,
			HwReg::Wave1 => self.wave_1,
			HwReg::Wave2 => self.wave_2,
			HwReg::Wave3 => self.wave_3,
			HwReg::Wave4 => self.wave_4,
			HwReg::Wave5 => self.wave_5,
			HwReg::Wave6 => self.wave_6,
			HwReg::Wave7 => self.wave_7,
			HwReg::Wave8 => self.wave_8,
			HwReg::Wave9 => self.wave_9,
			HwReg::WaveA => self.wave_a,
			HwReg::WaveB => self.wave_b,
			HwReg::WaveC => self.wave_c,
			HwReg::WaveD => self.wave_d,
			HwReg::WaveE => self.wave_e,
			HwReg::WaveF => self.wave_f,
			HwReg::Lcdc => self.lcdc,
			HwReg::Stat => self.stat,
			HwReg::Scy => self.scy,
			HwReg::Scx => self.scx,
			HwReg::Ly => self.ly,
			HwReg::Lyc => self.lyc,
			HwReg::Dma => self.dma,
			HwReg::Bgp => self.bgp,
			HwReg::Obp0 => self.obp0,
			HwReg::Obp1 => self.obp1,
			HwReg::Wy => self.wy,
			HwReg::Wx => self.wx,
			HwReg::Key1 => self.key1,
			HwReg::Vbk => self.vbk,
			HwReg::Hdma1 => self.hdma1,
			HwReg::Hdma2 => self.hdma2,
			HwReg::Hdma3 => self.hdma3,
			HwReg::Hdma4 => self.hdma4,
			HwReg::Hdma5 => self.hdma5,
			HwReg::Rp => self.rp,
			HwReg::Bcps => self.bcps,
			HwReg::Bcpd => self.bcpd,
			HwReg::Ocps => self.ocps,
			HwReg::Ocpd => self.ocpd,
			HwReg::Svbk => self.svbk,
			HwReg::Pcm12 => self.pcm12,
			HwReg::Pcm34 => self.pcm34,
			HwReg::Ie => self.ie,
		}
	}

	pub fn write(&mut self, register: HwReg, value: u8) {
		match register {
			HwReg::Ramg => self.ramg = value,
			HwReg::Romb0 => self.romb0 = value,
			HwReg::Romb1 => self.romb1 = value,
			HwReg::Ramb => self.ramb = value,
			HwReg::Rtclatch => self.rtclatch = value,
			HwReg::P1 => self.p1 = value,
			HwReg::Sb => self.sb = value,
			HwReg::Sc => self.sc = value,
			HwReg::Div => self.div = value,
			HwReg::Tima => self.tima = value,
			HwReg::Tma => self.tma = value,
			HwReg::Tac => self.tac = value,
			HwReg::If => self._if = value,
			HwReg::Nr10 => self.nr10 = value,
			HwReg::Nr11 => self.nr11 = value,
			HwReg::Nr12 => self.nr12 = value,
			HwReg::Nr13 => self.nr13 = value,
			HwReg::Nr14 => self.nr14 = value,
			HwReg::Nr21 => self.nr21 = value,
			HwReg::Nr22 => self.nr22 = value,
			HwReg::Nr23 => self.nr23 = value,
			HwReg::Nr24 => self.nr24 = value,
			HwReg::Nr30 => self.nr30 = value,
			HwReg::Nr31 => self.nr31 = value,
			HwReg::Nr32 => self.nr32 = value,
			HwReg::Nr33 => self.nr33 = value,
			HwReg::Nr34 => self.nr34 = value,
			HwReg::Nr41 => self.nr41 = value,
			HwReg::Nr42 => self.nr42 = value,
			HwReg::Nr43 => self.nr43 = value,
			HwReg::Nr44 => self.nr44 = value,
			HwReg::Nr50 => self.nr50 = value,
			HwReg::Nr51 => self.nr51 = value,
			HwReg::Nr52 => self.nr52 = value,
			HwReg::Wave0 => self.wave_0 = value,
			HwReg::Wave1 => self.wave_1 = value,
			HwReg::Wave2 => self.wave_2 = value,
			HwReg::Wave3 => self.wave_3 = value,
			HwReg::Wave4 => self.wave_4 = value,
			HwReg::Wave5 => self.wave_5 = value,
			HwReg::Wave6 => self.wave_6 = value,
			HwReg::Wave7 => self.wave_7 = value,
			HwReg::Wave8 => self.wave_8 = value,
			HwReg::Wave9 => self.wave_9 = value,
			HwReg::WaveA => self.wave_a = value,
			HwReg::WaveB => self.wave_b = value,
			HwReg::WaveC => self.wave_c = value,
			HwReg::WaveD => self.wave_d = value,
			HwReg::WaveE => self.wave_e = value,
			HwReg::WaveF => self.wave_f = value,
			HwReg::Lcdc => self.lcdc = value,
			HwReg::Stat => self.stat = value,
			HwReg::Scy => self.scy = value,
			HwReg::Scx => self.scx = value,
			HwReg::Ly => self.ly = value,
			HwReg::Lyc => self.lyc = value,
			HwReg::Dma => self.dma = value,
			HwReg::Bgp => self.bgp = value,
			HwReg::Obp0 => self.obp0 = value,
			HwReg::Obp1 => self.obp1 = value,
			HwReg::Wy => self.wy = value,
			HwReg::Wx => self.wx = value,
			HwReg::Key1 => self.key1 = value,
			HwReg::Vbk => self.vbk = value,
			HwReg::Hdma1 => self.hdma1 = value,
			HwReg::Hdma2 => self.hdma2 = value,
			HwReg::Hdma3 => self.hdma3 = value,
			HwReg::Hdma4 => self.hdma4 = value,
			HwReg::Hdma5 => self.hdma5 = value,
			HwReg::Rp => self.rp = value,
			HwReg::Bcps => self.bcps = value,
			HwReg::Bcpd => self.bcpd = value,
			HwReg::Ocps => self.ocps = value,
			HwReg::Ocpd => self.ocpd = value,
			HwReg::Svbk => self.svbk = value,
			HwReg::Pcm12 => self.pcm12 = value,
			HwReg::Pcm34 => self.pcm34 = value,
			HwReg::Ie => self.ie = value,
		}
	}

	pub fn new() -> Mmio {
		Default::default()
	}
}
