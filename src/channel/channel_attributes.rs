use crate::prelude::*;

pub enum ChannelAttribute {
    /// The sample rate, 100 (min) to 100000 (max), 0 = original rate (when the channel was created).
    Frequency,
    /// The panning/balance position, -1 (full left) to +1 (full right), 0 = centre.
    Pan,
    /// The volume level, 0 (silent) to 1 (full) or above.
    Volume,
    /// Sample rate conversion quality.
    Source,
    /// EAX wet/dry mix, 0 (full dry) to 1 (full wet), -1 = automatically calculate the mix based on the distance (the default).
    EaxMix,

    #[cfg(feature="bass_fx")] 
    Tempo(TempoAttribute),
}
impl Into<u32> for ChannelAttribute {
    fn into(self) -> u32 {
        use ChannelAttribute::*;

        match self {
            Frequency => BASS_ATTRIB_FREQ,
            Pan => BASS_ATTRIB_PAN,
            Volume => BASS_ATTRIB_VOL,
            Source => BASS_ATTRIB_SRC,
            EaxMix => BASS_ATTRIB_EAXMIX,
            
            #[cfg(feature="bass_fx")]
            Tempo(t) => t.into()
        }
    }
}

#[cfg(feature="bass_fx")] 
pub enum TempoAttribute {
    #[cfg(feature="bass_fx")] 
    /// The tempo of a channel, [-95%...0...+5000%] percents.
    Tempo,
    #[cfg(feature="bass_fx")] 
    /// The pitch of a channel, [-60...0...+60] semitones.
    TempoPitch,
    #[cfg(feature="bass_fx")] 
    /// The sample rate of a channel in Hz, but calculates by the same % as `TEMPO`.
    TempoFrequency,
    #[cfg(feature="bass_fx")] 
    /// Use the AA Filter for a tempo channel
    TempoUseAAFilter,
    #[cfg(feature="bass_fx")] 
    /// Sets the AA Filter length in taps (between 8 and 128).
    TempoAAFilterLength,
    #[cfg(feature="bass_fx")] 
    /// Use the tempo quick algorithm for a tempo channel (true =1, false =0).
    TempoUseQuickAlgorithm,
    #[cfg(feature="bass_fx")] 
    /// Sets the tempo sequence in ms. of a tempo channel (default = 82).
    TempoSequenceMs,
    #[cfg(feature="bass_fx")] 
    /// Sets the tempo seek window in ms. of a tempo channel (default = 82).
    TempoSeekWindowMs,
    #[cfg(feature="bass_fx")] 
    /// Sets the tempo overlap in ms. of a tempo channel (default = 12).
    TempoOverlapMs,
    #[cfg(feature="bass_fx")] 
    /// Sets the playback direction of a reverse channel (-1=reverse, 1=forward, or use one of the BASSFXReverse flags).
    ReverseDir,
}