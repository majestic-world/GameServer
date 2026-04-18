fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/favicon.ico");
        res.set("FileDescription", "Game Server Laucher");
        res.set("ProductName", "Game Server Laucher By Mk");
        res.set("OriginalFilename", "GameServer.exe");
        res.set("LegalCopyright", "Copyright (c) Mk");
        res.set("FileVersion", "1.0.0.0");
        res.set("ProductVersion", "1.0.0.0");
        res.set("CompanyName", "Majestic World Studio");
        res.compile().unwrap();
    }
}