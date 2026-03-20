# Adapter Status

Status values: `done` | `todo` (pure serial, portable) | `sdk` (requires vendor SDK/HID/DLL)

| C++ directory | Rust crate | Status | Notes |
|---|---|---|---|
| `89NorthLDI` | `mm-adapter-ldi` | done | ASCII `\n`, dynamic wavelengths |
| `AAAOTF` | — | todo | Crystal Technology AOTF, serial |
| `ABS` | — | sdk | Demo/test only |
| `AMF` | — | sdk | No serial interface found |
| `AOTF` | — | sdk | Uses inpout.dll (Windows LPT port I/O) |
| `ASIFW1000` | `mm-adapter-asifw1000` | done | Binary filter wheel |
| `ASIStage` | `mm-adapter-asi-stage` | done | `:A`/`:N` ASCII |
| `ASITiger` | `mm-adapter-asi-tiger` | done | `:A`/`:N` ASCII, 115200 baud |
| `ASIWPTR` | — | todo | ASI W-PTR serial device |
| `AgilentLaserCombiner` | — | sdk | LaserCombinerSDK.h |
| `Aladdin` | `mm-adapter-aladdin` | done | ASCII `\r` |
| `AlliedVisionCamera` | — | sdk | Vimba SDK |
| `AmScope` | — | sdk | Camera SDK |
| `Andor` | — | sdk | Andor SDK |
| `AndorLaserCombiner` | — | sdk | AB_ALC_REV64.dll |
| `AndorSDK3` | — | sdk | Andor SDK3 |
| `AndorShamrock` | — | sdk | Andor SDK |
| `Aquinas` | — | todo | Microfluidics controller, serial (LGPL) |
| `Aravis` | — | sdk | GLib/GObject/GigE |
| `Arduino` | `mm-adapter-arduino` | done | ASCII `\r` |
| `Arduino32bitBoards` | — | todo | 32-bit Arduino variant, serial |
| `ArduinoCounter` | — | todo | Arduino counter, serial |
| `Atik` | — | sdk | Atik camera SDK |
| `BDPathway` | — | sdk | BD Pathway imaging system |
| `BH_DCC_DCU` | — | sdk | Becker-Hickl photon counting |
| `Basler` | — | sdk | Pylon SDK |
| `BaumerOptronic` | — | sdk | Baumer SDK |
| `BlueboxOptics_niji` | `mm-adapter-niji` | done | Binary sync + `\r\n` |
| `CARVII` | `mm-adapter-carvii` | done | Single-char ASCII `\r` |
| `CNCMicroscope` | — | sdk | Custom hardware |
| `CSUW1` | `mm-adapter-csuw1` | done | CSV ASCII `\r` |
| `CairnOptoSpinUCSF` | — | sdk | Cairn/UCSF custom |
| `Cephla` | — | sdk | Cephla controller |
| `ChuoSeiki_MD5000` | `mm-adapter-chuoseiki` | done | ASCII `\r` |
| `ChuoSeiki_QT` | — | todo | ChuoSeiki QT-series stages, serial |
| `Cobolt` | `mm-adapter-cobolt` | done | ASCII `\r` |
| `CoboltOfficial` | — | todo | Cobolt vendor-independent variant, serial |
| `CoherentCube` | `mm-adapter-coherent-cube` | done | ASCII `\r` |
| `CoherentOBIS` | `mm-adapter-coherent-obis` | done | ASCII `\r` |
| `CoherentScientificRemote` | — | todo | Coherent Scientific Remote, serial |
| `Conix` | `mm-adapter-conix` | done | `:A`/`:N` ASCII |
| `CoolLEDpE300` | `mm-adapter-coolled` | done | CSS format |
| `CoolLEDpE4000` | `mm-adapter-coolled-pe4000` | done | CSS format, 4-channel |
| `Corvus` | `mm-adapter-corvus` | done | ASCII space-terminated |
| `DTOpenLayer` | — | sdk | DAQ hardware I/O |
| `DahengGalaxy` | — | sdk | Daheng SDK |
| `DemoCamera` | `mm-adapter-demo` | done | Simulated |
| `DirectElectron` | — | sdk | Direct Electron camera SDK |
| `Diskovery` | — | todo | Intelligent Imaging spinning disk, serial |
| `Dragonfly` | — | sdk | Andor Dragonfly SDK |
| `ESP32` | — | todo | ESP32 Arduino controller, serial |
| `ETL` | — | todo | Electrically Tunable Lens, serial |
| `Elveflow` | — | sdk | Elveflow microfluidics SDK |
| `EvidentIX85` | — | sdk | Evident/Olympus IX85 SDK |
| `EvidentIX85Win` | — | sdk | Evident/Olympus SDK (Windows) |
| `EvidentIX85XYStage` | — | sdk | Evident/Olympus SDK |
| `FLICamera` | — | sdk | FLI SDK |
| `FakeCamera` | — | sdk | Internal simulation utility |
| `Fli` | — | sdk | FLI SDK |
| `Fluigent` | — | sdk | Fluigent microfluidics SDK |
| `FocalPoint` | — | sdk | Prior FocalPoint |
| `FreeSerialPort` | — | sdk | Utility serial port device |
| `GenericSLM` | — | sdk | Generic SLM utility |
| `GigECamera` | — | sdk | GigE Vision SDK |
| `HIDManager` | — | sdk | USB HID |
| `HamiltonMVP` | `mm-adapter-hamilton-mvp` | done | `0x06` ACK binary |
| `Hikrobot` | — | sdk | MVSDK |
| `HydraLMT200` | — | todo | Hydra LMT-200 motion controller, serial |
| `IDSPeak` | — | sdk | IDS Peak SDK |
| `IDS_uEye` | — | sdk | IDS uEye SDK |
| `IIDC` | — | sdk | FireWire IIDC SDK |
| `ITC18` | — | sdk | Heka ITC-18 I/O hardware |
| `IlluminateLEDArray` | — | todo | LED array, serial + JSON (rapidjson) |
| `ImageProcessorChain` | — | sdk | Utility/aggregator |
| `IntegratedLaserEngine` | — | sdk | Andor ILE SDK |
| `IsmatecMCP` | `mm-adapter-ismatec` | done | Address-prefixed `*`-ACK |
| `JAI` | — | sdk | JAI camera SDK |
| `K8055` | — | sdk | Velleman K8055 USB HID |
| `K8061` | — | sdk | Velleman K8061 USB HID |
| `KuriosLCTF` | — | sdk | Thorlabs Windows DLLs only |
| `LaserQuantumLaser` | `mm-adapter-laser-quantum` | done | ASCII `\r` |
| `LeicaDMI` | — | todo | Leica DMI inverted microscope, serial |
| `LeicaDMR` | — | todo | Leica DMR upright microscope, serial |
| `LeicaDMSTC` | — | sdk | Leica DMSTC (check protocol) |
| `LightSheetManager` | — | sdk | Utility/aggregator |
| `Ludl` | `mm-adapter-ludl` | done | `:A` ASCII |
| `LudlLow` | — | todo | Low-level Ludl variant, serial |
| `Lumencor` | — | sdk | LightEngineAPI vendor SDK |
| `LumencorCIA` | — | todo | Lumencor CIA LED, serial |
| `LumencorSpectra` | `mm-adapter-lumencor-spectra` | done | Binary write-only |
| `Lumenera` | — | sdk | Lumenera camera SDK |
| `MCCDAQ` | — | sdk | Measurement Computing NI-DAQ |
| `MCL_MicroDrive` | — | sdk | Mad City Labs SDK |
| `MCL_NanoDrive` | — | sdk | Mad City Labs SDK |
| `MP285` | `mm-adapter-sutter-stage` | done | Sutter MP-285, `:A` ASCII |
| `MPBLaser` | — | todo | MPB Communications fiber laser, serial |
| `MT20` | — | sdk | Leica MT20 (check protocol) |
| `MaestroServo` | — | sdk | Check protocol |
| `Marzhauser` | `mm-adapter-marzhauser` | done | ASCII `\r` |
| `Marzhauser-LStep` | — | todo | Marzhauser LStep variant, serial |
| `MarzhauserLStepOld` | — | todo | Older LStep variant, serial |
| `MatrixVision` | — | sdk | Matrix Vision camera SDK |
| `MeadowlarkLC` | — | sdk | usbdrvd.h USB HID driver |
| `MicroFPGA` | — | todo | FPGA controller, USB serial |
| `MicroPoint` | — | sdk | Andor MicroPoint SDK |
| `Mightex` | — | sdk | Mightex camera SDK |
| `Mightex_BLS` | — | sdk | Mightex LED SDK |
| `Mightex_C_Cam` | — | sdk | Mightex camera SDK |
| `Mightex_SB_Cam` | — | sdk | Mightex camera SDK |
| `Modbus` | — | sdk | Modbus industrial protocol library |
| `Motic` | — | sdk | Motic camera SDK |
| `MoticMicroscope` | — | sdk | Motic SDK |
| `Motic_mac` | — | sdk | Motic SDK (macOS) |
| `NI100X` | — | sdk | National Instruments DAQ |
| `NIDAQ` | — | sdk | National Instruments DAQ |
| `NIMultiAnalog` | — | sdk | National Instruments DAQ |
| `NKTSuperK` | — | todo | NKT Photonics SuperK, serial |
| `NeoPixel` | — | todo | NeoPixel LED array, serial |
| `Neos` | `mm-adapter-neos` | done | No-response serial |
| `NewportCONEX` | `mm-adapter-newport-stage` | done | ASCII `\r\n` |
| `NewportSMC` | `mm-adapter-newport-stage` | done | ASCII `\r\n` |
| `Nikon` | — | sdk | Nikon vendor protocol (SDK) |
| `NikonKs` | — | sdk | Nikon Ks SDK |
| `NikonTE2000` | — | sdk | Nikon TE2000 SDK |
| `NotificationTester` | — | sdk | Internal test utility |
| `OVP_ECS2` | — | sdk | Check protocol |
| `ObjectiveImaging` | — | sdk | Check protocol |
| `Okolab` | — | sdk | okolib.h vendor SDK |
| `Omicron` | `mm-adapter-omicron` | done | `?CMD`/`!CMD` hex `\r` |
| `OpenCVgrabber` | — | sdk | OpenCV library |
| `OpenFlexure` | — | todo | OpenFlexure stage, serial |
| `OpenUC2` | — | todo | UC2 Arduino controller, serial |
| `Oxxius` | — | todo | Oxxius LaserBoxx single laser, serial |
| `OxxiusCombiner` | `mm-adapter-oxxius` | done | ASCII `\r`, L6Cc combiner |
| `PCO_Generic` | — | sdk | PCO camera SDK |
| `PI` | — | sdk | PI (Physik Instrumente) SDK |
| `PICAM` | — | sdk | Princeton Instruments SDK |
| `PIEZOCONCEPT` | — | sdk | Check protocol |
| `PI_GCS` | — | sdk | PI GCS SDK |
| `PI_GCS_2` | — | sdk | PI GCS SDK |
| `PVCAM` | — | sdk | Photometrics PVCAM SDK |
| `ParallelPort` | — | sdk | Windows parallel port I/O |
| `PeCon2000` | `mm-adapter-pecon` | done | Raw 3-byte BCD |
| `Pecon` | `mm-adapter-pecon` | done | Raw 3-byte BCD |
| `PicardStage` | — | sdk | Check protocol |
| `Piezosystem_30DV50` | — | todo | Piezosystem 30DV50, serial |
| `Piezosystem_NV120_1` | — | todo | Piezosystem NV-120/1, serial |
| `Piezosystem_NV40_1` | — | todo | Piezosystem NV-40/1, serial |
| `Piezosystem_NV40_3` | — | todo | Piezosystem NV-40/3, serial |
| `Piezosystem_dDrive` | — | todo | Piezosystem dDrive, serial |
| `Piper` | — | sdk | Check protocol |
| `Pixelink` | — | sdk | Pixelink camera SDK |
| `PlayerOne` | — | sdk | Player One Astronomy SDK |
| `PointGrey` | — | sdk | FLIR/PointGrey FlyCapture SDK |
| `PrecisExcite` | `mm-adapter-precis-excite` | done | ASCII `\r` |
| `Prior` | `mm-adapter-prior` | done | ASCII `\r` |
| `PriorLegacy` | — | todo | Legacy Prior variant, serial |
| `PriorPureFocus` | — | todo | Prior PureFocus, serial |
| `PrizmatixDevice` | — | todo | Prizmatix LED illuminator, serial |
| `PyDevice` | — | sdk | Python binding |
| `QCam` | — | sdk | QImaging SDK |
| `QSI` | — | sdk | QSI camera SDK |
| `Rapp` | — | sdk | obsROE_Device vendor class (serial wrapper) |
| `RappLasers` | — | sdk | Rapp laser SDK |
| `Rapp_UGA42` | — | sdk | Rapp UGA-42 vendor class |
| `RaptorEPIX` | — | sdk | Raptor EPIX SDK |
| `ReflectionFocus` | — | sdk | Check protocol |
| `Revealer` | — | sdk | Check protocol |
| `Sapphire` | `mm-adapter-sapphire` | done | ASCII `\r` |
| `Scientifica` | `mm-adapter-scientifica` | done | ASCII `\r` |
| `ScientificaMotion8` | — | todo | Scientifica Motion8 variant, serial |
| `ScionCam` | — | sdk | Scion camera SDK |
| `ScopeLED` | — | todo | ScopeLED illuminator, serial |
| `Sensicam` | — | sdk | PCO Sensicam SDK |
| `SequenceTester` | — | sdk | Internal test utility |
| `SerialManager` | — | sdk | Utility serial port manager |
| `SigmaKoki` | — | sdk | StCamD.h camera SDK |
| `SimpleCam` | — | sdk | Camera utility |
| `Skyra` | — | sdk | Cobolt Skyra SDK |
| `SmarActHCU-3D` | — | sdk | SmarAct SDK |
| `SouthPort` | — | sdk | Check protocol |
| `SpectralLMM5` | `mm-adapter-spectral-lmm5` | done | Hex-encoded binary `\r` |
| `Spinnaker` | — | sdk | FLIR Spinnaker SDK |
| `SpinnakerC` | — | sdk | FLIR Spinnaker SDK (C) |
| `Spot` | — | sdk | Spot camera SDK |
| `Standa` | — | sdk | Standa 8SMC SDK |
| `Standa8SMC4` | — | sdk | Standa 8SMC4 SDK |
| `StandaStage` | — | sdk | Standa SDK |
| `StarlightXpress` | — | sdk | Starlight Xpress camera SDK |
| `SutterLambda` | `mm-adapter-sutter-lambda` | done | Binary |
| `SutterLambda2` | — | todo | Sutter Lambda 2 (newer protocol), serial |
| `SutterLambdaParallelArduino` | — | todo | Sutter Lambda + Arduino parallel, serial |
| `SutterStage` | `mm-adapter-sutter-stage` | done | `:A` ASCII |
| `TCPIPPort` | — | sdk | TCP/IP utility |
| `TISCam` | — | sdk | TIS camera SDK |
| `TSI` | — | sdk | Thorlabs Scientific Imaging SDK |
| `TUCam` | — | sdk | Tucsen camera SDK |
| `TeensyPulseGenerator` | — | todo | Teensy serial pulse generator |
| `TeesnySLM` | — | sdk | Teensy SLM (check) |
| `ThorlabsAPTStage` | — | sdk | Thorlabs APT SDK |
| `ThorlabsCHROLIS` | — | todo | Thorlabs CHROLIS LED, serial |
| `ThorlabsDC40` | — | sdk | TLDC2200.h vendor SDK |
| `ThorlabsDCxxxx` | — | sdk | TLDC2200.h vendor SDK |
| `ThorlabsElliptecSlider` | `mm-adapter-elliptec` | done | Hex-position `\r` |
| `ThorlabsFilterWheel` | `mm-adapter-thorlabs-fw` | done | ASCII `\r` |
| `ThorlabsPM100x` | — | todo | Thorlabs PM100x power meter, serial |
| `ThorlabsSC10` | — | todo | Thorlabs SC10 shutter controller, serial |
| `ThorlabsTSP01` | — | todo | Thorlabs TSP01 temp/humidity, serial |
| `ThorlabsUSBCamera` | — | sdk | Thorlabs camera SDK |
| `Thorlabs_ELL14` | — | todo | Thorlabs ELL14 rotation stage, serial |
| `Tofra` | `mm-adapter-tofra` | done | IMS MDrive ASCII `\r` |
| `Toptica_iBeamSmartCW` | — | todo | Toptica iBeam Smart CW laser, serial |
| `TriggerScope` | — | todo | Trigger scope controller, serial |
| `TriggerScopeMM` | — | todo | TriggerScope MM variant, serial |
| `TwainCamera` | — | sdk | TWAIN SDK |
| `TwoPhoton` | — | sdk | Custom two-photon hardware |
| `USBManager` | — | sdk | USB utility |
| `USB_Viper_QPL` | — | sdk | USB HID |
| `UniversalMMHubSerial` | — | todo | Universal serial hub, serial |
| `UniversalMMHubUsb` | — | sdk | Universal USB hub |
| `UserDefinedSerial` | — | todo | User-defined serial device |
| `Utilities` | — | sdk | Utility devices (StateDeviceShutter, etc.) |
| `VariLC` | — | todo | Cambridge Research VariLC liquid crystal, serial |
| `VarispecLCTF` | `mm-adapter-varispec` | done | ASCII `\r` |
| `VisiTech_iSIM` | — | sdk | VisiTech iSIM SDK |
| `Vincent` | `mm-adapter-vincent` | done | ASCII `\r` |
| `Vortran` | `mm-adapter-vortran` | done | ASCII `\r` |
| `WOSM` | — | sdk | Check protocol |
| `WieneckeSinske` | — | todo | Wienecke & Sinske stage, serial |
| `XCite120PC_Exacte` | `mm-adapter-xcite` | done | ASCII `\r` |
| `XCiteLed` | — | todo | X-Cite LED variant, serial |
| `XCiteXT600` | — | todo | X-Cite XT600 variant, serial |
| `XLight` | — | todo | CrestOptics X-Light spinning disk, serial |
| `XLightV3` | — | todo | CrestOptics X-Light V3, serial |
| `Xcite` | `mm-adapter-xcite` | done | ASCII `\r` |
| `Ximea` | — | sdk | Ximea camera SDK |
| `YodnE600` | — | todo | Yodn E600 LED, serial |
| `Yokogawa` | — | todo | Yokogawa spinning disk, serial |
| `ZWO` | — | sdk | ZWO ASI camera SDK |
| `Zaber` | `mm-adapter-zaber` | done | ASCII `\n` (Zaber ASCII v2) |
| `ZeissAxioZoom` | — | sdk | Zeiss SDK |
| `ZeissCAN` | — | sdk | Zeiss CAN bus SDK |
| `ZeissCAN29` | — | sdk | Zeiss CAN29 SDK |
| `dc1394` | — | sdk | FireWire DC1394 |
| `iSIMWaveforms` | — | sdk | iSIM waveform utility |
| `kdv` | — | sdk | Check protocol |
| `nPoint` | — | sdk | nPoint piezo SDK |
| `pgFocus` | — | todo | pgFocus autofocus, serial |
