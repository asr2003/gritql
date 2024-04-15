use marzano_util::{
    diff::{parse_modified_ranges, FileDiff},
    position::FileRange,
};
use std::{fs::File, io::Read, path::PathBuf};

pub fn run_git_diff(path: &PathBuf) -> Result<String> {
pub fn extract_modified_ranges(diff_path: &PathBuf) -> Result<Vec<FileDiff>> {
pub(crate) fn extract_target_ranges(
    arg: &Option<Option<PathBuf>>,
) -> Result<Option<Vec<FileRange>>> {
    let raw_diff = if let Some(Some(diff_path)) = &arg {
        extract_modified_ranges(diff_path)?
    } else if let Some(None) = &arg {
        let diff = run_git_diff(&std::env::current_dir()?)?;
        parse_modified_ranges(&diff)?
    } else {
        return Ok(None);
    Ok(Some(
        raw_diff
            .into_iter()
            .flat_map(|diff| match diff.new_path {
                Some(new_path) => {
                    let mapped = diff.after.into_iter().map(move |range| FileRange {
                        range,
                        file_path: new_path.clone(),
                    });
                    mapped.collect::<Vec<_>>()
                }
                None => {
                    log::info!("Skipping diff with no new path: {:?}", diff);
                    vec![]
                }
            })
            .collect(),
    ))