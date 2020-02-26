use azure_devops_api::request::Method;
use azure_devops_api::work::iterations::post_team_iteration;

#[test]
fn post_team_iteration_then_build() {
    let request = post_team_iteration("fake_org", "fake_proj", "fake_id")
        .unwrap()
        .build()
        .unwrap();

    assert_eq!(request.get_method(), &Method::Post);
    assert_eq!(request.get_url().as_str(), "https://dev.azure.com/fake_org/fake_proj/_apis/work/teamsettings/iterations?api-version=5.1");
    assert_eq!(request.get_body(), r#"{"id":"fake_id"}"#)
}

// Useful for create or update iterations
// &String::from(concat!(
//     r#"{"#,
//         r#""id":"fake_id","#,
//         r#""name":"fake_name","#,
//         r#""path":"fake_path","#,
//         r#""attributes":{"#,
//             r#""startDate":null,"#,
//             r#""finishDate":null,"#,
//             r#""timeFrame":"fake_time_frame""#,
//         r#"},"#,
//         r#""url":"fake_url""#,
//         r#"}"#
//     ))
//  };
