slint::include_modules!();
const TAXPE: f64 =0.30;
const OWNPE: f64 =0.55;
const PROFITPE: f64 =0.55;
const OPREXP: f64 =0.10;

fn main() -> Result<(), slint::PlatformError> {
    let ui:AppWindow  = AppWindow::new()?;

    let ui_handle:slint::Weak<AppWindow>  = ui.as_weak();
    ui.on_divide_income(move |string|{
        let ui:AppWindow =ui_handle.unwrap();
        let num: f64=string.trim().parse().unwrap();
        let tax: f64=num*TAXPE;
        let owner: f64=num*OWNPE;
        let profit: f64=num*PROFITPE;
        let op_ex: f64=num*OPREXP;
        let result:String=format!("Taxes: {:2}\nOwner: {:2}\nProfit: {:2}\nOperating expenses: {:2}",tax,owner,profit,op_ex);

        ui.set_results(result.into());
        
    });

    ui.run()?;

    Ok(())
}
