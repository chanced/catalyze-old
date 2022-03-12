use catalyze::{util::Generic, visit::Visitor, ToCase, ToPascalCase, ToShoutySnakeCase};

pub struct V {}

impl<'a, U> Visitor<'a, U> for V {
    type Error = &'static str;

    fn visit_node(&mut self, node: catalyze::Node<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_node(self, node)
    }

    fn visit_package(&mut self, pkg: catalyze::Package<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_package(self, pkg)
    }

    fn visit_file(&mut self, f: catalyze::File<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_file(self, f)
    }

    fn visit_message(&mut self, msg: catalyze::Message<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_message(self, msg)
    }

    fn visit_enum(&mut self, enm: catalyze::Enum<'a, U>) -> Result<(), Self::Error> {
        // for n in enm.nodes() {}
        catalyze::visit::visit_enum(self, enm)
    }

    fn visit_enum_value(&mut self, val: catalyze::EnumValue<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_extension(&mut self, ext: catalyze::Extension<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof(&mut self, oneof: catalyze::Oneof<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_oneof(self, oneof)
    }

    fn visit_real_oneof(&mut self, oneof: catalyze::Oneof<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_synthetic_oneof(&mut self, oneof: catalyze::Oneof<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_service(&mut self, svc: catalyze::Service<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_service(self, svc)
    }

    fn visit_method(&mut self, mth: catalyze::Method<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_field(&mut self, fld: catalyze::Field<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_field(self, fld)
    }

    fn visit_scalar_field(&mut self, fld: catalyze::ScalarField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_embed_field(&mut self, fld: catalyze::EmbedField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_enum_field(&mut self, fld: catalyze::EnumField<'a, U>) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_map_field(&mut self, fld: catalyze::MapField<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_map_field(self, fld)
    }

    fn visit_mapped_scalar_field(
        &mut self,
        fld: catalyze::MappedScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapped_enum_field(
        &mut self,
        fld: catalyze::MappedEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_mapped_embed_field(
        &mut self,
        fld: catalyze::MappedEmbedField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_field(
        &mut self,
        fld: catalyze::RepeatedField<'a, U>,
    ) -> Result<(), Self::Error> {
        catalyze::visit::visit_repeated_field(self, fld)
    }

    fn visit_repeated_scalar_field(
        &mut self,
        fld: catalyze::RepeatedScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_enum_field(
        &mut self,
        fld: catalyze::RepeatedEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_repeated_embed_field(
        &mut self,
        fld: catalyze::RepeatedEmbedField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof_field(&mut self, fld: catalyze::OneofField<'a, U>) -> Result<(), Self::Error> {
        catalyze::visit::visit_oneof_field(self, fld)
    }

    fn visit_oneof_scalar_field(
        &mut self,
        fld: catalyze::OneofScalarField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof_embed_field(
        &mut self,
        fld: catalyze::OneofEmbedField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_oneof_enum_field(
        &mut self,
        fld: catalyze::OneofEnumField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }

    fn visit_real_oneof_field(
        &mut self,
        fld: catalyze::OneofField<'a, U>,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}
