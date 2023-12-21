use protobuf::reflect::FileDescriptor;

use crate::comments::Comments;
use crate::enum_::{AllEnums, Enum};
use crate::error::Error;
use crate::extension::Extension;
use crate::iter::Iter;
use crate::message::{AllMessages, Message};
use crate::node::{AllNodes, Container, Node, Nodes};
use crate::package::{Package, WeakPackage};

use crate::service::Service;
use crate::uninterpreted_option::UninterpretedOption;
use crate::*;
use std::cell::RefCell;

use std::collections::{HashSet, VecDeque};
use std::path::PathBuf;
use std::rc::{Rc, Weak};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Syntax {
    Proto2,
    Proto3,
}

impl Syntax {
    pub fn supports_required_prefix(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto2(&self) -> bool {
        match self {
            Syntax::Proto2 => true,
            Syntax::Proto3 => false,
        }
    }
    pub fn is_proto3(&self) -> bool {
        match self {
            Syntax::Proto2 => false,
            Syntax::Proto3 => true,
        }
    }
}

impl TryFrom<String> for Syntax {
    type Error = Error;

    fn try_from(v: String) -> Result<Self, Self::Error> {
        match &*v.to_lowercase() {
            "proto2" => Ok(Syntax::Proto2),
            "proto3" => Ok(Syntax::Proto3),
            "" => Ok(Syntax::Proto2),
            _ => Err(Error::invalid_syntax(v)),
        }
    }
}

impl ToString for Syntax {
    fn to_string(&self) -> String {
        match self {
            Syntax::Proto2 => "proto2",
            Syntax::Proto3 => "proto3",
        }
        .to_string()
    }
}
impl From<&str> for Syntax {
    fn from(v: &str) -> Self {
        match v.to_lowercase().as_str() {
            "proto2" => Syntax::Proto2,
            "proto3" => Syntax::Proto3,
            _ => Syntax::Proto2,
        }
    }
}

/// Each of the definitions above may have "options" attached.  These are
/// just annotations which may cause code to be generated slightly differently
/// or may contain hints for code that manipulates protocol messages.
///
/// Clients may define custom options as extensions of the *Options messages.
/// These extensions may not yet be known at parsing time, so the parser cannot
/// store the values in them.  Instead it stores them in a field in the *Options
/// message called uninterpreted_option. This field must have the same name
/// across all *Options messages. We then use this field to populate the
/// extensions when we build a descriptor, at which point all protos have been
/// parsed and so all extensions are known.
///
/// Extension numbers for custom options may be chosen as follows:
/// * For options which will only be used within a single application or
///   organization, or for experimental options, use field numbers 50000
///   through 99999.  It is up to you to ensure that you do not use the
///   same number for multiple options.
/// * For options which will be published and used publicly by multiple
///   independent entities, e-mail protobuf-global-extension-registry@google.com
///   to reserve extension numbers. Simply provide your project name (e.g.
///   Objective-C plugin) and your project website (if available) -- there's no
///   need to explain how you intend to use them. Usually you only need one
///   extension number. You can declare multiple options with only one extension
///   number by putting them in a sub-message. See the Custom Options section of
///   the docs for examples:
///   <https://developers.google.com/protocol-buffers/docs/proto#options>
///   If this turns out to be popular, a web service will be set up
///   to automatically assign option numbers.
#[derive(Debug, Clone, Copy)]
pub struct Options<'a> {
    opts: Option<&'a protobuf::descriptor::FileOptions>,
}

impl<'a> From<Option<&'a protobuf::descriptor::FileOptions>> for Options<'a> {
    fn from(opts: Option<&'a protobuf::descriptor::FileOptions>) -> Self {
        Self { opts }
    }
}

impl<'a> Options<'a> {
    /// Java package where classes generated from this .proto will be
    /// placed.  By default, the proto package is used, but this is often
    /// inappropriate because proto packages do not normally start with backwards
    /// domain names.
    pub fn java_package(&self) -> Option<&str> {
        self.opts.and_then(|opts| opts.java_package.as_ref())
    }
    /// If set, all the classes from the .proto file are wrapped in a single
    /// outer class with the given name.  This applies to both Proto1
    /// (equivalent to the old "--one_java_file" option) and Proto2 (where
    /// a .proto always translates to a single class, but you may want to
    /// explicitly choose the class name).
    pub fn java_outer_classname(&self) -> Option<&str> {
        self.opts
            .and_then(|opts| opts.java_outer_classname.as_ref())
    }

    /// If set true, then the Java code generator will generate a separate .java
    /// file for each top-level message, enum, and service defined in the .proto
    /// file.  Thus, these types will *not* be nested inside the outer class
    /// named by java_outer_classname.  However, the outer class will still be
    /// generated to contain the file's getDescriptor() method as well as any
    /// top-level extensions defined in the file.
    pub fn java_multiple_files(&self) -> bool {
        self.opts
            .and_then(|opts| opts.java_multiple_files)
            .unwrap_or(false)
    }

    /// If set true, then the Java2 code generator will generate code that
    /// throws an exception whenever an attempt is made to assign a non-UTF-8
    /// byte sequence to a string field.
    ///
    /// Message reflection will do the same.
    /// However, an extension field still accepts non-UTF-8 byte sequences.
    ///
    /// This option has no effect on when used with the lite runtime.
    pub fn java_string_check_utf8(&self) -> bool {
        self.opts
            .map(|opts| opts.java_string_check_utf8())
            .unwrap_or(false)
    }
    /// Generated classes can be optimized for speed or code size.
    pub fn optimize_for(&self) -> OptimizeMode {
        self.opts().optimize_for().into()
    }
    /// Sets the Go package where structs generated from this .proto will be
    /// placed. If omitted, the Go package will be derived from the following:
    ///   - The basename of the package import path, if provided.
    ///   - Otherwise, the package statement in the .proto file, if present.
    ///   - Otherwise, the basename of the .proto file, without extension.
    pub fn go_package(&self) -> &str {
        self.opts().go_package()
    }
    /// Should generic services be generated in each language?  "Generic" services
    /// are not specific to any particular RPC system.  They are generated by the
    /// main code generators in each language (without additional plugins).
    /// Generic services were the only kind of service generation supported by
    /// early versions of google.protobuf.
    ///
    /// Generic services are now considered deprecated in favor of using plugins
    /// that generate code specific to your particular RPC system.  Therefore,
    /// these default to false.  Old code which depends on generic services should
    /// explicitly set them to true.
    pub fn cc_generic_services(&self) -> bool {
        self.opts().cc_generic_services()
    }

    pub fn java_generic_services(&self) -> bool {
        self.opts().java_generic_services()
    }

    pub fn py_generic_services(&self) -> bool {
        self.opts().py_generic_services()
    }

    pub fn php_generic_services(&self) -> bool {
        self.opts().php_generic_services()
    }

    /// Is this file deprecated?
    /// Depending on the target platform, this can emit Deprecated annotations
    /// for everything in the file, or it will be completely ignored; in the very
    /// least, this is a formalization for deprecating files.
    pub fn deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    pub fn is_deprecated(&self) -> bool {
        self.opts().deprecated()
    }
    /// Enables the use of arenas for the proto messages in this file. This applies
    /// only to generated classes for C++.
    pub fn cc_enable_arenas(&self) -> bool {
        self.opts().cc_enable_arenas()
    }
    /// Sets the objective c class prefix which is prepended to all objective c
    /// generated classes from this .proto. There is no default.
    pub fn objc_class_prefix(&self) -> &str {
        self.opts().objc_class_prefix()
    }

    /// Namespace for generated classes; defaults to the package.
    pub fn csharp_namespace(&self) -> &str {
        self.opts().csharp_namespace()
    }
    /// By default Swift generators will take the proto package and CamelCase it
    /// replacing '.' with underscore and use that to prefix the types/symbols
    /// defined. When this options is provided, they will use this value instead
    /// to prefix the types/symbols defined.
    pub fn swift_prefix(&self) -> &str {
        self.opts().swift_prefix()
    }

    /// Sets the php class prefix which is prepended to all php generated classes
    /// from this .proto. Default is empty.
    pub fn php_class_prefix(&self) -> &str {
        self.opts().php_class_prefix()
    }

    /// Use this option to change the namespace of php generated classes. Default
    /// is empty. When this option is empty, the package name will be used for
    /// determining the namespace.
    pub fn php_namespace(&self) -> &str {
        self.opts().php_namespace()
    }

    /// Use this option to change the namespace of php generated metadata classes.
    /// Default is empty. When this option is empty, the proto file name will be
    /// used for determining the namespace.
    pub fn php_metadata_namespace(&self) -> &str {
        self.opts().php_metadata_namespace()
    }
    /// Use this option to change the package of ruby generated classes. Default
    /// is empty. When this option is not set, the package name will be used for
    /// determining the ruby package.
    pub fn ruby_package(&self) -> &str {
        self.opts().ruby_package()
    }
    /// The parser stores options it doesn't recognize here.
    /// See the documentation for the "Options" section above.
    pub fn uninterpreted_options(&self) -> &[UninterpretedOption] {
        (&self.opts().uninterpreted_option).into()
    }
}

#[derive(Debug, Clone)]
pub struct File(Rc<Detail>);

#[derive(Debug, Clone)]
struct Detail {
    descriptor: FileDescriptor,
    file_path: PathBuf,
    fqn: String,
    messages: RefCell<Vec<Message>>,
    enums: RefCell<Vec<Enum>>,
    services: RefCell<Vec<Service>>,
    defined_extensions: RefCell<Vec<Extension>>,
    build_target: bool,
    pkg_comments: RefCell<Comments>,
    comments: RefCell<Comments>,

    pkg: WeakPackage,
    dependents: RefCell<Vec<WeakFile>>,
    imports: RefCell<Vec<WeakFile>>,
    used_imports: Rc<RefCell<HashSet<String>>>,
    syntax: Syntax,
}

impl Detail {
    pub fn new(build_target: bool, descriptor: FileDescriptor, pkg: Package) -> Rc<Self> {
        let fqn = match descriptor.package() {
            "" => String::default(),
            p => format!(".{}", p),
        };
        Rc::new(Self {
            descriptor,
            pkg: pkg.into(),
            build_target,
            fqn,
            syntax: descriptor.syntax(),
            file_path: PathBuf::from(descriptor.name()),
            dependents: Rc::new(RefCell::new(Vec::new())),
            imports: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.dependencies().len(),
            ))),
            defined_extensions: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.extensions().len(),
            ))),
            messages: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.messages().len(),
            ))),
            enums: Rc::new(RefCell::new(Vec::with_capacity(descriptor.enums().len()))),
            services: Rc::new(RefCell::new(Vec::with_capacity(
                descriptor.services().len(),
            ))),
            pkg_comments: RefCell::new(Comments::default()),
            comments: RefCell::new(Comments::default()),
            used_imports: Rc::new(RefCell::new(HashSet::new())),
        })
    }
}

impl File {
    pub(crate) fn new(
        build_target: bool,
        desc: FileDescriptor,
        pkg: Package,
    ) -> Result<Self, Error> {
        let file = Self(Detail::new(build_target, desc, pkg))
            .hydrate_messages()?
            .hydrate_enums()
            .hydrate_services()
            .hydrate_extensions()
            .assign_comments();
        Ok(file)
    }

    fn assign_comments(self) -> Self {
        {
            for loc in self.descriptor().source_code_info() {
                match loc.file_descriptor_path() {
                    Ok(p) => match p {
                        FileDescriptorPath::Package => self.set_package_comments(loc.into()),
                        FileDescriptorPath::Syntax => self.set_comments(loc.into()),
                        _ => {
                            let n = self.node_at_path(loc.path());
                            if let Some(n) = n {
                                n.set_comments(loc.into())
                            }
                        }
                    },
                    Err(_) => continue,
                }
            }
        }
        self
    }

    fn hydrate_extensions(self) -> Self {
        {
            let mut exts = self.0.defined_extensions.borrow_mut();
            let container = self.as_container();
            for ed in self.descriptor().extensions() {
                let ext = Extension::new(ed, container.clone());
                exts.push(ext);
            }
        }
        self
    }
    fn hydrate_messages(self) -> Result<Self, Error> {
        {
            let container = self.clone().as_container();
            let mut msgs = self.0.messages.borrow_mut();
            for md in self.descriptor().messages() {
                let msg = Message::new(md, container.clone())?;
                msgs.push(msg);
            }
        }
        Ok(self)
    }
    fn hydrate_enums(self) -> Self {
        {
            let container = self.as_container();

            let mut enums = self.0.enums.borrow_mut();
            for ed in self.descriptor().enums() {
                let e = Enum::new(ed, container.clone());
                enums.push(e);
            }
        }
        self
    }
    fn hydrate_services(self) -> Self {
        {
            let mut services = self.0.services.borrow_mut();
            for sd in self.descriptor().services() {
                let svc = Service::new(sd, self.clone());
                services.push(svc);
            }
        }
        self
    }
    pub fn fully_qualified_name(&self) -> &str {
        &self.0.fqn
    }
    pub fn as_container(&self) -> Container {
        self.clone().into()
    }
    pub fn name(&self) -> &str {
        &self.0.descriptor.name()
    }
    pub fn package(&self) -> Package {
        self.0.pkg.clone().into()
    }
    pub fn build_target(&self) -> bool {
        self.0.build_target
    }
    pub fn comments(&self) -> Comments {
        *self.0.comments.borrow()
    }
    pub fn file_path(&self) -> &PathBuf {
        &self.0.file_path
    }
    /// Returns comments attached to the package in this File if any exist.
    pub fn package_comments(&self) -> Comments {
        *self.0.pkg_comments.borrow()
    }
    pub fn descriptor(&self) -> FileDescriptor {
        self.0.descriptor
    }
    pub(crate) fn set_comments(&self, comments: Comments) {
        *self.0.comments.borrow_mut() = comments;
    }
    pub(crate) fn set_package_comments(&self, comments: Comments) {
        *self.0.pkg_comments.borrow_mut() = comments;
    }
    pub fn message(&self, name: &str) -> Option<Message> {
        let name = name.to_lowercase();
        self.all_messages().find(|m| {
            m.name().to_lowercase() == name || m.fully_qualified_name().to_lowercase() == name
        })
    }
    pub fn messages(&self) -> Iter<Message> {
        Iter::from(&self.0.messages)
    }
    pub fn enum_(&self, name: &str) -> Option<Enum> {
        self.all_enums().find(|e| e.name() == name)
    }
    pub fn enum_s(&self) -> Iter<Enum> {
        self.enums()
    }
    pub fn enums(&self) -> Iter<Enum> {
        Iter::from(&self.0.enums)
    }
    pub fn services(&self) -> Iter<Service> {
        Iter::from(&self.0.services)
    }
    pub fn defined_extensions(&self) -> Iter<Extension> {
        Iter::from(&self.0.defined_extensions)
    }

    pub fn imports(&self) -> FileRefs {
        self.0.imports.clone().into()
    }

    pub fn dependents(&self) -> FileRefs {
        self.0.dependents.clone().into()
    }
    pub fn transitive_imports(&self) -> TransitiveImports {
        TransitiveImports::new(self.0.imports.clone())
    }

    /// all_messages returns an iterator of all top-level and nested messages from this
    /// file.
    pub fn all_messages(&self) -> AllMessages {
        AllMessages::new(self.0.messages.clone())
    }

    /// all_enums returns an iterator of all top-level and nested enums from this file.
    pub fn all_enums(&self) -> AllEnums {
        AllEnums::new(self.0.enums.clone(), self.0.messages.clone())
    }

    pub fn path(&self) -> PathBuf {
        self.0.file_path.clone()
    }

    pub(crate) fn add_import(&self, file: File) {
        self.0.imports.borrow_mut().push(file.into());
    }

    pub(crate) fn mark_import_as_used(&self, file: File) {
        self.0
            .used_imports
            .borrow_mut()
            .insert(file.fully_qualified_name().to_string());
    }

    pub(crate) fn add_dependent(&self, file: File) {
        self.0.dependents.borrow_mut().push(file.into());
    }
    fn downgrade(&self) -> WeakFile {
        WeakFile(Rc::downgrade(&self.0))
    }
    pub fn all_nodes(&self) -> AllNodes {
        AllNodes::new(self.into())
    }
    pub fn nodes(&self) -> Nodes {
        Nodes::new(vec![
            self.enums().into(),
            self.messages().into(),
            self.services().into(),
            self.defined_extensions().into(),
        ])
    }

    pub fn unused_imports(&self) -> Vec<File> {
        let used_imports = self.0.used_imports.borrow();
        let mut unused_imports = Vec::new();
        for file in self.0.imports.borrow().iter() {
            if !used_imports.contains(&file.fully_qualified_name()) {
                unused_imports.push(file.into());
            }
        }
        unused_imports
    }

    pub fn syntax(&self) -> Syntax {
        self.0.syntax
    }

    pub(crate) fn node_at_path(&self, path: &[i32]) -> Option<Node> {
        if path.is_empty() {
            return Some(self.into());
        }
        if path.len() % 2 == 1 {
            return None;
        }
        let next = path[1] as usize;
        FileDescriptorPath::try_from(path[0]).ok().and_then(|p| {
            match p {
                FileDescriptorPath::MessageType => self
                    .0
                    .messages
                    .borrow()
                    .get(next)
                    .cloned()
                    .map(Node::Message),
                FileDescriptorPath::EnumType => {
                    self.0.enums.borrow().get(next).cloned().map(Node::Enum)
                }
                FileDescriptorPath::Service => self
                    .0
                    .services
                    .borrow()
                    .get(next)
                    .cloned()
                    .map(Node::Service),
                _ => None,
            }
            .and_then(|n| n.node_at_path(&path[2..]))
        })
    }

    pub fn service(&self, name: &str) -> Option<Service> {
        self.services().find(|s| s.name() == name)
    }
}

impl From<&WeakFile> for File {
    fn from(weak: &WeakFile) -> Self {
        weak.upgrade()
    }
}
impl From<WeakFile> for File {
    fn from(weak: WeakFile) -> Self {
        weak.upgrade()
    }
}

impl PartialEq for File {
    fn eq(&self, other: &Self) -> bool {
        self.file_path() == other.file_path()
    }
}

impl PartialEq<WeakFile> for File {
    fn eq(&self, other: &WeakFile) -> bool {
        self.file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug, Clone)]
pub(crate) struct WeakFile(Weak<Detail>);

impl WeakFile {
    pub fn fully_qualified_name(&self) -> &str {
        self.upgrade().fully_qualified_name()
    }
    pub fn package(&self) -> Package {
        self.upgrade().package()
    }

    pub fn build_target(&self) -> bool {
        self.upgrade().build_target()
    }
    fn upgrade(&self) -> File {
        File(self.0.upgrade().expect("Failed to upgrade weak file"))
    }
}

impl From<File> for WeakFile {
    fn from(file: File) -> Self {
        file.downgrade()
    }
}
impl From<&File> for WeakFile {
    fn from(file: &File) -> Self {
        file.downgrade()
    }
}
impl PartialEq<File> for WeakFile {
    fn eq(&self, other: &File) -> bool {
        self.upgrade().file_path() == other.file_path()
    }
}
impl PartialEq for WeakFile {
    fn eq(&self, other: &Self) -> bool {
        self.upgrade().file_path() == other.upgrade().file_path()
    }
}

#[derive(Debug, Clone)]
pub struct TransitiveImports {
    queue: VecDeque<File>,
    processed: HashSet<String>,
}
impl TransitiveImports {
    pub(crate) fn new(files: RefCell<Vec<WeakFile>>) -> Self {
        Self {
            queue: VecDeque::from_iter(files.borrow().iter().map(|f| f.into())),
            processed: HashSet::new(),
        }
    }
}
impl Iterator for TransitiveImports {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        while let Some(file) = self.queue.pop_front() {
            if !self.processed.contains(file.name()) {
                self.processed.insert(file.name().to_string());
                for d in file.imports() {
                    self.queue.push_back(d);
                }
                return Some(file);
            }
        }
        None
    }
}

/// An iterator that upgrades weak file references to `File`s.
pub struct FileRefs {
    files: Rc<RefCell<Vec<WeakFile>>>,
    index: usize,
}
impl FileRefs {
    pub fn len(&self) -> usize {
        self.files.borrow().len()
    }
    pub fn is_empty(&self) -> bool {
        self.files.borrow().is_empty()
    }
    pub fn empty() -> Self {
        Self {
            files: Rc::new(RefCell::new(Vec::new())),
            index: 0,
        }
    }
}

impl From<&Rc<RefCell<Vec<WeakFile>>>> for FileRefs {
    fn from(files: &Rc<RefCell<Vec<WeakFile>>>) -> Self {
        FileRefs {
            files: files.clone(),
            index: 0,
        }
    }
}
impl From<Rc<RefCell<Vec<WeakFile>>>> for FileRefs {
    fn from(files: Rc<RefCell<Vec<WeakFile>>>) -> Self {
        FileRefs {
            files: files.clone(),
            index: 0,
        }
    }
}

impl From<WeakFile> for FileRefs {
    fn from(file: WeakFile) -> Self {
        FileRefs {
            files: Rc::new(RefCell::new(vec![file])),
            index: 0,
        }
    }
}

impl From<Option<WeakFile>> for FileRefs {
    fn from(file: Option<WeakFile>) -> Self {
        file.map_or(Self::empty(), Into::into)
    }
}
impl Iterator for FileRefs {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        let files = self.files.borrow();
        if let Some(file) = files.get(self.index) {
            self.index += 1;
            return Some(file.into());
        }
        None
    }
}

/// Generated classes can be optimized for speed or code size.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[repr(i32)]
pub enum OptimizeMode {
    /// Generate complete code for parsing, serialization,
    Speed = 1,
    /// etc.
    ///
    /// Use ReflectionOps to implement these methods.
    CodeSize = 2,
    /// Generate code using MessageLite and the lite runtime.
    LiteRuntime = 3,
}

impl From<protobuf::descriptor::file_options::OptimizeMode> for OptimizeMode {
    fn from(value: protobuf::descriptor::file_options::OptimizeMode) -> Self {
        match value {
            protobuf::descriptor::file_options::OptimizeMode::SPEED => OptimizeMode::Speed,
            protobuf::descriptor::file_options::OptimizeMode::CODE_SIZE => OptimizeMode::CodeSize,
            protobuf::descriptor::file_options::OptimizeMode::LITE_RUNTIME => {
                OptimizeMode::LiteRuntime
            }
        }
    }
}

impl TryFrom<i32> for OptimizeMode {
    type Error = Error;
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(OptimizeMode::Speed),
            2 => Ok(OptimizeMode::CodeSize),
            3 => Ok(OptimizeMode::LiteRuntime),
            _ => Err(Error::invalid_optimize_mode(value)),
        }
    }
}
