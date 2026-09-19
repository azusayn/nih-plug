use nih_plug::prelude::*;
use std::sync::Arc;

#[derive(Default)]
struct Mono {
    params: Arc<MonoParams>,
}

#[derive(Params)]
struct MonoParams {
    #[id = "mono"]
    pub mono: BoolParam,
}

impl Default for MonoParams {
    fn default() -> Self {
        Self {
            mono: BoolParam::new("Mono", true),
        }
    }
}

impl Plugin for Mono {
    const NAME: &'static str = "Mono";
    const VENDOR: &'static str = "Example";
    const URL: &'static str = "";
    const EMAIL: &'static str = "";
    const VERSION: &'static str = "0.1.0";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),
        ..AudioIOLayout::const_default()
    }];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        if !self.params.mono.value() {
            return ProcessStatus::Normal;
        }

        for mut frame in buffer.iter_samples() {
            if frame.len() < 2 {
                break;
            }

            let left = *frame.get_mut(0).unwrap();
            let right = *frame.get_mut(1).unwrap();
            let mono = (left + right) * 0.5;

            *frame.get_mut(0).unwrap() = mono;
            *frame.get_mut(1).unwrap() = mono;
        }

        ProcessStatus::Normal
    }
}

impl ClapPlugin for Mono {
    const CLAP_ID: &'static str = "com.example.mono";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Sum stereo to mono");
    const CLAP_MANUAL_URL: Option<&'static str> = None;
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for Mono {
    const VST3_CLASS_ID: [u8; 16] = *b"ExampleMonoPlug!";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Tools];
}

nih_export_clap!(Mono);
nih_export_vst3!(Mono);
