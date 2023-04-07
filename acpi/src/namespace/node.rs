use super::{
    impl_core_display,
    objects::{
        Device, Method, Mutex, Name, OperationRegion, PowerResource, Processor, ThermalZone,
    },
    Children, Display, Scope,
};
use alloc::rc::Rc;
use core::cell::RefCell;

pub(crate) enum Node<'a> {
    Device(Device<'a>),
    Method(Method<'a>),
    Mutex(Mutex<'a>),
    Name(Name<'a>),
    OperationRegion(OperationRegion<'a>),
    PowerResource(PowerResource<'a>),
    Processor(Processor<'a>),
    Scope(Scope<'a>),
    ThermalZone(ThermalZone<'a>),
}

impl<'a> Node<'a> {
    pub(crate) fn parent(&self) -> Option<Rc<RefCell<Node<'a>>>> {
        match self {
            Node::Device(device) => device.parent(),
            Node::Method(method) => method.parent(),
            Node::Mutex(mutex) => mutex.parent(),
            Node::Name(name) => name.parent(),
            Node::OperationRegion(operation_region) => operation_region.parent(),
            Node::PowerResource(power_resource) => power_resource.parent(),
            Node::Processor(processor) => processor.parent(),
            Node::Scope(scope) => scope.parent(),
            Node::ThermalZone(thermal_zone) => thermal_zone.parent(),
        }
    }

    pub(crate) fn name(&self) -> Option<[u8; 4]> {
        Some(match self {
            Node::Device(device) => device.name(),
            Node::Method(method) => method.name(),
            Node::Mutex(mutex) => mutex.name(),
            Node::Name(name) => name.name(),
            Node::OperationRegion(operation_region) => operation_region.name(),
            Node::PowerResource(power_resource) => power_resource.name(),
            Node::Processor(processor) => processor.name(),
            Node::ThermalZone(thermal_zone) => thermal_zone.name(),

            Node::Scope(scope) => return scope.name(),
        })
    }

    pub(crate) fn children(&self) -> Option<&Children<'a>> {
        match self {
            Node::Device(device) => Some(device.children()),
            Node::PowerResource(power_resource) => Some(power_resource.children()),
            Node::Processor(processor) => Some(processor.children()),
            Node::Scope(scope) => Some(scope.children()),
            Node::ThermalZone(thermal_zone) => Some(thermal_zone.children()),

            Node::Method(_) | Node::Mutex(_) | Node::Name(_) | Node::OperationRegion(_) => None,
        }
    }

    pub(crate) fn children_mut(&mut self) -> Option<&mut Children<'a>> {
        match self {
            Node::Device(device) => Some(device.children_mut()),
            Node::PowerResource(power_resource) => Some(power_resource.children_mut()),
            Node::Processor(processor) => Some(processor.children_mut()),
            Node::Scope(scope) => Some(scope.children_mut()),
            Node::ThermalZone(thermal_zone) => Some(thermal_zone.children_mut()),

            Node::Method(_) | Node::Mutex(_) | Node::Name(_) | Node::OperationRegion(_) => None,
        }
    }
}

impl<'a> Display for Node<'a> {
    fn display(&self, f: &mut core::fmt::Formatter, depth: usize, last: bool) -> core::fmt::Result {
        match self {
            Node::Device(device) => device.display(f, depth, last),
            Node::Method(method) => method.display(f, depth, last),
            Node::Mutex(mutex) => mutex.display(f, depth, last),
            Node::Name(name) => name.display(f, depth, last),
            Node::OperationRegion(operation_region) => operation_region.display(f, depth, last),
            Node::PowerResource(power_resource) => power_resource.display(f, depth, last),
            Node::Processor(processor) => processor.display(f, depth, last),
            Node::Scope(scope) => scope.display(f, depth, last),
            Node::ThermalZone(thermal_zone) => thermal_zone.display(f, depth, last),
        }
    }
}

impl_core_display!(Node);
