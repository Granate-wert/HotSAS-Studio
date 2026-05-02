use crate::ApplicationError;
use hotsas_core::{FormulaDefinition, FormulaPack, FormulaPackMetadata};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone)]
pub struct FormulaRegistryService {
    formulas: BTreeMap<String, FormulaDefinition>,
    pack_metadata: Vec<FormulaPackMetadata>,
}

impl FormulaRegistryService {
    pub fn new(packs: Vec<FormulaPack>) -> Result<Self, ApplicationError> {
        if packs.is_empty() {
            return Err(ApplicationError::InvalidFormulaPack(
                "at least one formula pack is required".to_string(),
            ));
        }

        let mut formulas = BTreeMap::new();
        let mut pack_metadata = Vec::new();

        for pack in packs {
            validate_pack(&pack)?;
            let categories = pack
                .formulas
                .iter()
                .map(|formula| formula.category.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect::<Vec<_>>();
            pack_metadata.push(FormulaPackMetadata {
                pack_id: pack.pack_id.clone(),
                title: pack.title.clone(),
                version: pack.version.clone(),
                formula_count: pack.formulas.len(),
                categories,
            });

            for formula in pack.formulas {
                if formulas.contains_key(&formula.id) {
                    return Err(ApplicationError::DuplicateFormulaId(formula.id));
                }
                formulas.insert(formula.id.clone(), formula);
            }
        }

        Ok(Self {
            formulas,
            pack_metadata,
        })
    }

    pub fn list_formulas(&self) -> Vec<FormulaDefinition> {
        self.formulas.values().cloned().collect()
    }

    pub fn get_formula(&self, id: &str) -> Result<FormulaDefinition, ApplicationError> {
        self.formulas
            .get(id)
            .cloned()
            .ok_or_else(|| ApplicationError::FormulaNotFound(id.to_string()))
    }

    pub fn list_by_category(&self, category: &str) -> Vec<FormulaDefinition> {
        self.formulas
            .values()
            .filter(|formula| formula.category == category)
            .cloned()
            .collect()
    }

    pub fn list_categories(&self) -> Vec<String> {
        self.formulas
            .values()
            .map(|formula| formula.category.clone())
            .collect::<BTreeSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn get_pack_metadata(&self) -> Vec<FormulaPackMetadata> {
        self.pack_metadata.clone()
    }

    pub fn get_linked_template_id(
        &self,
        formula_id: &str,
    ) -> Result<Option<String>, ApplicationError> {
        Ok(self.get_formula(formula_id)?.linked_circuit_template_id)
    }

    pub fn validate_formula_bindings(
        &self,
        known_template_ids: &[String],
    ) -> Result<(), ApplicationError> {
        let known = known_template_ids.iter().collect::<BTreeSet<_>>();
        for formula in self.formulas.values() {
            if let Some(template_id) = &formula.linked_circuit_template_id {
                if !known.contains(template_id) {
                    return Err(ApplicationError::InvalidBinding(format!(
                        "formula {} references unknown template {}",
                        formula.id, template_id
                    )));
                }
            }
        }
        Ok(())
    }
}

fn validate_pack(pack: &FormulaPack) -> Result<(), ApplicationError> {
    require_non_empty(&pack.pack_id, "pack_id")?;
    require_non_empty(&pack.title, "title")?;
    require_non_empty(&pack.version, "version")?;
    if pack.formulas.is_empty() {
        return Err(ApplicationError::InvalidFormulaPack(
            "formulas must not be empty".to_string(),
        ));
    }

    for formula in &pack.formulas {
        require_non_empty(&formula.id, "formula.id")?;
        require_non_empty(&formula.title, "formula.title")?;
        require_non_empty(&formula.category, "formula.category")?;
        if formula.equations.is_empty() {
            return Err(ApplicationError::InvalidFormulaPack(format!(
                "formula {} must contain at least one equation",
                formula.id
            )));
        }
        if formula.outputs.is_empty() {
            return Err(ApplicationError::InvalidFormulaPack(format!(
                "formula {} must contain at least one output",
                formula.id
            )));
        }
        for name in formula.variables.keys() {
            require_non_empty(name, "variable name")?;
        }
        for name in formula.outputs.keys() {
            require_non_empty(name, "output name")?;
        }
    }

    Ok(())
}

fn require_non_empty(value: &str, field: &str) -> Result<(), ApplicationError> {
    if value.trim().is_empty() {
        return Err(ApplicationError::InvalidFormulaPack(format!(
            "{field} must not be empty"
        )));
    }
    Ok(())
}
