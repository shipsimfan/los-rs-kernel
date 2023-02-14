#[repr(usize)]
#[derive(Clone, Copy)]
pub enum ExceptionType {
    DivisionError = 0,
    Debug,
    NonMaskableInterrupt,
    Breakpoint,
    Overflow,
    BoundRangeExceeded,
    InvalidOpcode,
    DeviceNotAvailable,
    DoubleFault,
    InvalidTSS = 10,
    SegmentNotPresent,
    StackSegmentFault,
    GeneralProtectionFault,
    PageFault,
    X87FloatingPointException = 16,
    AlignmentCheck,
    MachineCheck,
    SIMDFloatingPointException,
    VirtualizationException,
    ControlProtectionException,
    HypervisorInjectionException = 28,
    VMMCommunicationException,
    SecurityException,
}

impl ExceptionType {
    pub(super) fn parse(interrupt: u64) -> Option<Self> {
        match interrupt {
            x if x == ExceptionType::DivisionError as u64 => Some(ExceptionType::DivisionError),
            x if x == ExceptionType::Debug as u64 => Some(ExceptionType::Debug),
            x if x == ExceptionType::NonMaskableInterrupt as u64 => {
                Some(ExceptionType::NonMaskableInterrupt)
            }
            x if x == ExceptionType::Breakpoint as u64 => Some(ExceptionType::Breakpoint),
            x if x == ExceptionType::Overflow as u64 => Some(ExceptionType::Overflow),
            x if x == ExceptionType::BoundRangeExceeded as u64 => {
                Some(ExceptionType::BoundRangeExceeded)
            }
            x if x == ExceptionType::InvalidOpcode as u64 => Some(ExceptionType::InvalidOpcode),
            x if x == ExceptionType::DeviceNotAvailable as u64 => {
                Some(ExceptionType::DeviceNotAvailable)
            }
            x if x == ExceptionType::DoubleFault as u64 => Some(ExceptionType::DoubleFault),
            x if x == ExceptionType::InvalidTSS as u64 => Some(ExceptionType::InvalidTSS),
            x if x == ExceptionType::SegmentNotPresent as u64 => {
                Some(ExceptionType::SegmentNotPresent)
            }
            x if x == ExceptionType::StackSegmentFault as u64 => {
                Some(ExceptionType::StackSegmentFault)
            }
            x if x == ExceptionType::GeneralProtectionFault as u64 => {
                Some(ExceptionType::GeneralProtectionFault)
            }
            x if x == ExceptionType::PageFault as u64 => Some(ExceptionType::PageFault),
            x if x == ExceptionType::X87FloatingPointException as u64 => {
                Some(ExceptionType::X87FloatingPointException)
            }
            x if x == ExceptionType::AlignmentCheck as u64 => Some(ExceptionType::AlignmentCheck),
            x if x == ExceptionType::MachineCheck as u64 => Some(ExceptionType::MachineCheck),
            x if x == ExceptionType::SIMDFloatingPointException as u64 => {
                Some(ExceptionType::SIMDFloatingPointException)
            }
            x if x == ExceptionType::VirtualizationException as u64 => {
                Some(ExceptionType::VirtualizationException)
            }
            x if x == ExceptionType::ControlProtectionException as u64 => {
                Some(ExceptionType::ControlProtectionException)
            }
            x if x == ExceptionType::HypervisorInjectionException as u64 => {
                Some(ExceptionType::HypervisorInjectionException)
            }
            x if x == ExceptionType::VMMCommunicationException as u64 => {
                Some(ExceptionType::VMMCommunicationException)
            }
            x if x == ExceptionType::SecurityException as u64 => {
                Some(ExceptionType::SecurityException)
            }
            _ => None,
        }
    }
}

impl core::fmt::Display for ExceptionType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ExceptionType::DivisionError => "Division Error",
                ExceptionType::Debug => "Debug",
                ExceptionType::NonMaskableInterrupt => "Non Maskable Interrupt",
                ExceptionType::Breakpoint => "Breakpoint",
                ExceptionType::Overflow => "Overflow",
                ExceptionType::BoundRangeExceeded => "Bound Range Exceeded",
                ExceptionType::InvalidOpcode => "Invalid Opcode",
                ExceptionType::DeviceNotAvailable => "Device Not Available",
                ExceptionType::DoubleFault => "Double Fault",
                ExceptionType::InvalidTSS => "Invalid TSS",
                ExceptionType::SegmentNotPresent => "Segment Not Present",
                ExceptionType::StackSegmentFault => "Stack Segment Fault",
                ExceptionType::GeneralProtectionFault => "General Protection Fault",
                ExceptionType::PageFault => "Page Fault",
                ExceptionType::X87FloatingPointException => "x87 Floating Point Exception",
                ExceptionType::AlignmentCheck => "Alignment Check",
                ExceptionType::MachineCheck => "Machine Check",
                ExceptionType::SIMDFloatingPointException => "SIMD Floating Point Exception",
                ExceptionType::VirtualizationException => "Virtualization Exception",
                ExceptionType::ControlProtectionException => "Control Protection Exception",
                ExceptionType::HypervisorInjectionException => "Hypervisor Injection Exception",
                ExceptionType::VMMCommunicationException => "VMM Communication Exception",
                ExceptionType::SecurityException => "Security Exception",
            }
        )
    }
}
