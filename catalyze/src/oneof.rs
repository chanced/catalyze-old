use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};

use protobuf::reflect::OneofDescriptor;

use crate::{
    comments::Comments,
    field::Field,
    file::{File, FileRefs, WeakFile},
    iter::Iter,
    message::{Message, WeakMessage},
    node::Node,
    package::Package,
    uninterpreted_option::UninterpretedOption,
};

#[derive(Debug, Clone)]
pub struct Options {
    // message fields
    ///  The parser stores options it doesn't recognize here. See above.
    // @@protoc_insertion_point(field:google.protobuf.OneofOptions.uninterpreted_option)
    pub uninterpreted_option: Vec<UninterpretedOption>,
    // special fields
    // @@protoc_insertion_point(special_field:google.protobuf.OneofOptions.special_fields)
    pub special_fields: protobuf::SpecialFields,
}
impl Options {
    /// The parser stores options it doesn't recognize here. See above.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        (&self.opts().uninterpreted_option).into()
    }
}
impl From<Option<&protobuf::descriptor::OneofOptions>> for Options {
    fn from(opts: Option<&protobuf::descriptor::OneofOptions>) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct OneofDetail {
    pub descriptor: OneofDescriptor,
    fqn: String,
    fields: Rc<RefCell<Vec<Field>>>,
    msg: WeakMessage,
    is_synthetic: bool,
    comments: RefCell<Comments>,
    imports: Rc<RefCell<Vec<WeakFile>>>,
}

#[derive(Debug, Clone)]
pub struct Oneof(Rc<OneofDetail>);

impl Oneof {
    pub fn new(descriptor: OneofDescriptor, msg: Message) -> Self {
        let fully_qualified_name = format!("{}.{}", msg.fully_qualified_name(), descriptor.name());
        Oneof(Rc::new(OneofDetail {
            descriptor,
            fqn: fully_qualified_name,
            fields: Rc::new(RefCell::new(Vec::default())),
            msg: msg.clone().into(),
            is_synthetic: true,
            comments: RefCell::new(Comments::default()),
            imports: Rc::new(RefCell::new(Vec::default())),
        }))
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub fn name(&self) -> &str {
        &self.0.name
    }
    pub fn fields(&self) -> Iter<Field> {
        Iter::from(&self.0.fields)
    }
    pub fn comments(&self) -> Comments {
        *self.0.comments.borrow()
    }
    pub fn message(&self) -> Message {
        self.0.msg.clone().into()
    }

    pub fn file(&self) -> File {
        self.0.msg.file()
    }
    pub fn descriptor(&self) -> OneofDescriptor {
        self.0.descriptor
    }
    pub fn package(&self) -> Package {
        self.file().package()
    }
    pub fn imports(&self) -> FileRefs {
        FileRefs::from(&self.0.imports)
    }
    pub(crate) fn add_field(&self, field: Field) {
        self.0.fields.borrow_mut().push(field.clone());
    }

    pub(crate) fn update_imports(&self) {
        for field in self.fields() {
            field
                .imports()
                .for_each(|i| self.0.imports.borrow_mut().push(i.into()))
        }
    }

    pub(crate) fn set_comments(&self, comments: Comments) {
        self.0.comments.replace(comments);
    }

    fn downgrade(&self) -> WeakOneof {
        WeakOneof(Rc::downgrade(&self.0))
    }

    pub fn is_real(&self) -> bool {
        !self.0.is_synthetic
    }
    pub fn is_synthetic(&self) -> bool {
        self.0.is_synthetic
    }
    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        if path.is_empty() {
            Some(Node::Oneof(self.clone()))
        } else {
            None
        }
    }
}

impl From<WeakOneof> for Oneof {
    fn from(oneof: WeakOneof) -> Self {
        oneof.upgrade()
    }
}
impl From<&WeakOneof> for Oneof {
    fn from(oneof: &WeakOneof) -> Self {
        oneof.upgrade()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakOneof(Weak<OneofDetail>);

impl WeakOneof {
    fn upgrade(&self) -> Oneof {
        Oneof(self.0.upgrade().expect("Failed to upgrade Oneof"))
    }
}
impl From<Oneof> for WeakOneof {
    fn from(oneof: Oneof) -> Self {
        oneof.downgrade()
    }
}
impl From<&Oneof> for WeakOneof {
    fn from(oneof: &Oneof) -> Self {
        oneof.downgrade()
    }
}
