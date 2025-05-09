//! Module for formatting repository information with indicators
//!
//! This module provides functions for formatting repository names and descriptions
//! with visual indicators to help quickly identify their type.
//!
//! # Repository Display Format
//!
//! ## Status Indicators
//!
//! - (fork) or (fork: description) - Fork of another repository
//! - 🔒 - Private repository

use serde::{Deserialize, Serialize};

/// Repository source (GitHub or GitLab)
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum RepoSource {
    GitHub,
    GitLab,
}

/// Formats a repository name with private status indicator and source
pub fn format_repo_name(name: &str, _is_fork: bool, is_private: bool, source: RepoSource) -> String {
    // Add source and private icons
    let private_icon = if is_private { " 🔒" } else { "" };
    let source_icon = match source {
        RepoSource::GitHub => " [GH]",
        RepoSource::GitLab => " [GL]",
    };

    format!("{}{}{}", name, private_icon, source_icon)
}



/// Formats a complete repository display string with name and description
pub fn format_repository(name: &str, description: &str, is_fork: bool, is_private: bool, source: RepoSource) -> String {
    let formatted_name = format_repo_name(name, is_fork, is_private, source);

    if is_fork {
        if description.is_empty() {
            format!("{} (fork)", formatted_name)
        } else {
            // Trim the description before formatting
            let trimmed_description = description.trim();
            format!("{} (fork: {})", formatted_name, trimmed_description)
        }
    } else if description.is_empty() {
        formatted_name
    } else {
        // Trim the description before formatting
        let trimmed_description = description.trim();
        format!("{} ({})", formatted_name, trimmed_description)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_repo_name() {
        // Regular repository (GitHub)
        assert_eq!(format_repo_name("normal-repo", false, false, RepoSource::GitHub), "normal-repo [GH]");

        // Regular repository (GitLab)
        assert_eq!(format_repo_name("normal-repo", false, false, RepoSource::GitLab), "normal-repo [GL]");

        // Forked repository - fork status is now handled in format_repository
        assert_eq!(format_repo_name("forked-repo", true, false, RepoSource::GitHub), "forked-repo [GH]");

        // Private repository
        assert_eq!(format_repo_name("private-repo", false, true, RepoSource::GitHub), "private-repo 🔒 [GH]");

        // Both forked and private - fork status is now handled in format_repository
        assert_eq!(format_repo_name("private-fork", true, true, RepoSource::GitLab), "private-fork 🔒 [GL]");
    }



    #[test]
    fn test_format_repository() {
        // Repository with description (GitHub)
        assert_eq!(
            format_repository("web-app", "Frontend application", false, false, RepoSource::GitHub),
            "web-app [GH] (Frontend application)"
        );

        // Repository with description (GitLab)
        assert_eq!(
            format_repository("web-app", "Frontend application", false, false, RepoSource::GitLab),
            "web-app [GL] (Frontend application)"
        );

        // Repository with description and fork status
        assert_eq!(
            format_repository("forked-api", "Backend service", true, false, RepoSource::GitHub),
            "forked-api [GH] (fork: Backend service)"
        );

        // Repository with description and private status
        assert_eq!(
            format_repository("mobile-app", "iOS client", false, true, RepoSource::GitHub),
            "mobile-app 🔒 [GH] (iOS client)"
        );

        // Repository with description, fork and private status
        assert_eq!(
            format_repository("game-demo", "Unity project", true, true, RepoSource::GitLab),
            "game-demo 🔒 [GL] (fork: Unity project)"
        );

        // Repository with no description
        assert_eq!(
            format_repository("test-framework", "", false, false, RepoSource::GitHub),
            "test-framework [GH]"
        );

        // Repository with no description but with fork and private status
        assert_eq!(
            format_repository("private-fork", "", true, true, RepoSource::GitLab),
            "private-fork 🔒 [GL] (fork)"
        );

        // Repository with description containing extra whitespace
        assert_eq!(
            format_repository("whitespace-test", "  Description with extra spaces  ", false, false, RepoSource::GitHub),
            "whitespace-test [GH] (Description with extra spaces)"
        );

        // Forked repository with no description
        assert_eq!(
            format_repository("just-fork", "", true, false, RepoSource::GitLab),
            "just-fork [GL] (fork)"
        );
    }
}
