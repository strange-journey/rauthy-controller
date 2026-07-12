use kube::CustomResourceExt;
use rauthy_controller::controller;

fn main() {
    let mut crd = controller::OIDCClient::crd();
    if crd.spec.names.categories.as_ref().is_some_and(Vec::is_empty) {
        crd.spec.names.categories = None;
    }
    if crd.spec.names.short_names.as_ref().is_some_and(Vec::is_empty) {
        crd.spec.names.short_names = None;
    }
    for version in &mut crd.spec.versions {
        if version
            .additional_printer_columns
            .as_ref()
            .is_some_and(Vec::is_empty)
        {
            version.additional_printer_columns = None;
        }
    }

    crd.metadata
        .annotations
        .get_or_insert_with(Default::default)
        .insert("helm.sh/resource-policy".to_string(), "keep".to_string());

    print!("{}", serde_yaml::to_string(&crd).unwrap());
}
