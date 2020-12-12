use rusty_infer::errors::Error;
use rusty_infer::{copy_imgs, gen_anns, Config};

fn cfg_print_to(output_dir: &str) -> Config {
    let root = env!("CARGO_MANIFEST_DIR");
    Config {
        input_dir: format!("{}/tests/example", root),
        output_dir: format!("{}/tests/example/{}", root, output_dir),
        dataset_name: "mydataset".to_string(),
        label: "test".to_string(),
        img_dir: "X".to_string(),
        msk_dir: "Y".to_string(),
    }
}

#[test]
fn copies_ims() -> Result<(), Error> {
    let cfg = cfg_print_to("sly");
    gen_anns(&cfg)?;
    copy_imgs(&cfg);
    Ok(())
}
