use ldap3::*;

fn main() {
    let ldap = LdapConn::new("ldap://192.168.0.100:3268");
    let mut ldap_con = match ldap {
        Ok(l) => l,
        Err(e) => panic!("{}", e),
    };

    ldap_con.simple_bind("CN=Administrator,CN=Users,DC=tech69,DC=local", "Passw0rd").unwrap();

    let res = ldap_con.search("DC=tech69,DC=local", Scope::Subtree, "(objectclass=user)", vec!["dn"]).unwrap();

    let (re, _) = res.success().unwrap();

    for i in re {
        println!("{}", SearchEntry::construct(i).dn);
    }
}
