use crate::{
	ast::{Reference, Stmt, StmtKind},
	visit::{self, Visit},
};

/// Determine a list of all fields that are initialized in a class constructor.
#[derive(Default)]
pub struct VisitClassInit {
	pub fields: Vec<String>,
}

impl VisitClassInit {
	pub fn analyze_statements(&mut self, statements: &[Stmt]) {
		for stmt in statements {
			self.visit_stmt(stmt);
		}
	}
}

impl Visit<'_> for VisitClassInit {
	fn visit_stmt(&mut self, node: &Stmt) {
		match &node.kind {
			StmtKind::Assignment { variable, value: _ } => {
				visit::visit_reference(self, variable);
			}
			_ => (),
		}
		visit::visit_stmt(self, node);
	}

	fn visit_reference(&mut self, node: &Reference) {
		match node {
			Reference::InstanceMember { property, object: _ } => self.fields.push(property.name.clone()),
			_ => (),
		}
		visit::visit_reference(self, node);
	}
}
