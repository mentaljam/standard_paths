fn main() {
    let ids = vec![        
        ("FOLDERID_Desktop",        "B4BFCC3A-DB2C-424C-B029-7FE99A87C641"), // DesktopLocation
        ("FOLDERID_Documents",      "FDD39AD0-238F-46AF-ADB4-6C85480369C7"), // DocumentsLocation
        ("FOLDERID_Fonts",          "FD228CB7-AE11-4AE3-864C-16F3910AB8FE"), // FontsLocation
        ("FOLDERID_Programs",       "A77F5D77-2E2B-44C3-A6A2-ABA601054A51"), // ApplicationsLocation
        ("FOLDERID_Music",          "4BD8D571-6D19-48D3-BE97-422220080E43"), // MusicLocation
        ("FOLDERID_Videos",         "18989B1D-99B5-455B-841C-AB7C74E4DDFC"), // MoviesLocation
        ("FOLDERID_Pictures",       "33E28130-4E1E-4676-835A-98395C3BC3BB"), // PicturesLocation
        ("FOLDERID_Downloads",      "374DE290-123F-4565-9164-39C4925E467B"), // DownloadLocation
        ("FOLDERID_LocalAppData",   "F1B32785-6FBA-4FCF-9D55-7B8E7F157091"), // AppLocalDataLocation, AppLocalDataLocation,
                                                                             // GenericDataLocation, ConfigLocation,
                                                                             // GenericConfigLocation, AppConfigLocation
        ("FOLDERID_RoamingAppData", "3EB685DB-65F9-4CF6-A03A-E3EF65729F3D"), // AppDataLocation
        ("FOLDERID_ProgramData",    "62AB5D82-FDC1-4DC3-A9DD-070D1D495D97")  // ConfigLocation, AppConfigLocation,
                                                                             // AppDataLocation, AppLocalDataLocation,
                                                                             // GenericConfigLocation, GenericDataLocation
    ];

    for &(name, guid) in &ids {
        println!("\
/// https://msdn.microsoft.com/en-us/library/dd378457.aspx#{name}
#[allow(non_upper_case_globals)]
const {name}: GUID = GUID {{
    Data1: 0x{},
    Data2: 0x{},
    Data3: 0x{},
    Data4: [0x{}, 0x{}, 0x{}, 0x{}, 0x{}, 0x{}, 0x{}, 0x{}]
}};\n",
                 guid.get(..8).unwrap(),
                 guid.get(9..13).unwrap(),
                 guid.get(14..18).unwrap(),
                 guid.get(19..21).unwrap(),
                 guid.get(21..23).unwrap(),
                 guid.get(24..26).unwrap(),
                 guid.get(26..28).unwrap(),
                 guid.get(28..30).unwrap(),
                 guid.get(30..32).unwrap(),
                 guid.get(32..34).unwrap(),
                 guid.get(34..36).unwrap(),
                 name = name);
    }
}