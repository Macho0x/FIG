use fig_fsl::{
    generate_sbe_target, load_merged_trading_schema, CppCodegen, CsharpCodegen, FixYamlCodegen,
    GoCodegen, JsonSchemaCodegen, OcamlCodegen, Parser, ProtoCodegen, PythonCodegen, SbeTargetLang,
    SbeXmlCodegen, TypeScriptCodegen, ZigCodegen,
};

fn orders_schema() -> fig_fsl::Schema {
    load_merged_trading_schema()
}

fn minimal_schema() -> fig_fsl::Schema {
    let input = r#"
        schema test v1.0 {
            message NewOrderSingle {
                channel_type: request_response
                cl_ord_id: string(max_len: 20) @1
                order_qty: decimal64(precision: 0) @2
            }
        }
    "#;
    Parser::parse(input).unwrap()
}

#[test]
fn test_go_codegen_orders() {
    let schema = orders_schema();
    let code = GoCodegen::generate(&schema);

    assert!(code.contains("type NewOrderSingle struct"));
    assert!(code.contains("type CancelRequest struct"));
    assert!(code.contains("type ExecutionReport struct"));
    assert!(code.contains("type Side int32"));
    assert!(code.contains("type PriceLevel struct"));
    assert!(code.contains("`json:\"cl_ord_id\""));
    assert!(code.contains("package generated"));
}

#[test]
fn test_proto_codegen_orders() {
    let schema = orders_schema();
    let code = ProtoCodegen::generate(&schema);

    assert!(code.contains("syntax = \"proto3\""));
    assert!(code.contains("message NewOrderSingle"));
    assert!(code.contains("message ExecutionReport"));
    assert!(code.contains("cl_ord_id"));
    assert!(code.contains("package trading_orders"));
}

#[test]
fn test_sbe_xml_codegen_orders() {
    let schema = orders_schema();
    let code = SbeXmlCodegen::generate(&schema);

    assert!(code.contains("<sbe:messageSchema"));
    assert!(code.contains("messageSchema"));
    assert!(code.contains("<message name=\"NewOrderSingle\""));
    assert!(code.contains("<message name=\"CancelRequest\""));
    assert!(code.contains("id=\"1\""));
    assert!(code.contains("clOrdId"));
}

#[test]
fn test_cpp_codegen_orders() {
    let schema = orders_schema();
    let code = CppCodegen::generate(&schema);

    assert!(code.contains("struct NewOrderSingle"));
    assert!(code.contains("struct CancelRequest"));
    assert!(code.contains("struct ExecutionReport"));
    assert!(code.contains("struct PriceLevel"));
    assert!(code.contains("enum class Side"));
    assert!(code.contains("cl_ord_id"));
    assert!(code.contains("#include <string>"));
}

#[test]
fn test_csharp_codegen_orders() {
    let schema = orders_schema();
    let code = CsharpCodegen::generate(&schema);

    assert!(code.contains("public class NewOrderSingle"));
    assert!(code.contains("public class CancelRequest"));
    assert!(code.contains("public class ExecutionReport"));
    assert!(code.contains("public class PriceLevel"));
    assert!(code.contains("public enum Side"));
    assert!(code.contains("namespace Generated"));
    assert!(code.contains("ClOrdId"));
}

#[test]
fn test_go_codegen_minimal_inline() {
    let schema = minimal_schema();
    let code = GoCodegen::generate(&schema);

    assert!(code.contains("type NewOrderSingle struct"));
    assert!(code.contains("ClOrdId string `json:\"cl_ord_id\""));
    assert!(code.contains("OrderQty float64 `json:\"order_qty\""));
}

#[test]
fn test_proto_codegen_minimal_inline() {
    let schema = minimal_schema();
    let code = ProtoCodegen::generate(&schema);

    assert!(code.contains("message NewOrderSingle"));
    assert!(code.contains("string cl_ord_id = 1"));
    assert!(code.contains("double order_qty = 2"));
}

#[test]
fn test_python_codegen_orders() {
    let schema = orders_schema();
    let code = PythonCodegen::generate(&schema);
    assert!(code.contains("@dataclass"));
    assert!(code.contains("class NewOrderSingle"));
    assert!(code.contains("class Side(Enum)"));
    assert!(code.contains("class PriceLevel:"));
}

#[test]
fn test_typescript_codegen_orders() {
    let schema = orders_schema();
    let code = TypeScriptCodegen::generate(&schema);
    assert!(code.contains("export interface NewOrderSingle"));
    assert!(code.contains("export type Side ="));
    assert!(code.contains("export type ClientOrderId = string"));
    assert!(code.contains("NewOrderSingleChannelType"));
    assert!(code.contains("CBOR (snake_case keys"));
}

#[test]
fn test_ocaml_codegen_orders() {
    let schema = orders_schema();
    let code = OcamlCodegen::generate(&schema);
    assert!(code.contains("type NewOrderSingle ="));
    assert!(code.contains("type Side ="));
    assert!(code.contains("type ClientOrderId = string"));
}

#[test]
fn test_zig_codegen_orders() {
    let schema = orders_schema();
    let code = ZigCodegen::generate(&schema);
    assert!(code.contains("pub const NewOrderSingle"));
    assert!(code.contains("pub const Side = enum(u8)"));
    assert!(code.contains("pub const ClientOrderId = []const u8"));
}

#[test]
fn test_java_codegen_orders() {
    let schema = orders_schema();
    let code = fig_fsl::JavaCodegen::generate(&schema);
    assert!(code.contains("public class NewOrderSingle"));
    assert!(code.contains("public enum Side"));
    assert!(code.contains("NewOrderSingleChannelType"));
}

#[test]
fn test_json_schema_codegen_orders() {
    let schema = orders_schema();
    let code = JsonSchemaCodegen::generate(&schema);
    assert!(code.contains("\"$schema\""));
    assert!(code.contains("\"NewOrderSingle\""));
}

#[test]
fn test_fix_yaml_codegen_orders() {
    let schema = orders_schema();
    let code = FixYamlCodegen::generate(&schema);
    assert!(code.contains("fix_version"));
    assert!(code.contains("fig_message: NewOrderSingle"));
}

#[test]
fn test_sbe_target_go_orders() {
    let schema = orders_schema();
    let code = generate_sbe_target(&schema, SbeTargetLang::Go);
    assert!(code.contains("NewOrderSingleEncoder"));
    assert!(code.contains("SCHEMA_ID"));
    assert!(code.contains("package figsbe"));
}

#[test]
fn test_sbe_xml_template_ids_match_rust() {
    let schema = orders_schema();
    let xml = SbeXmlCodegen::generate(&schema);
    for (i, msg) in schema.messages.iter().enumerate() {
        let tid = i + 1;
        assert!(
            xml.contains(&format!("<message name=\"{}\" id=\"{}\">", msg.name, tid)),
            "missing template id for {}",
            msg.name
        );
    }
}
