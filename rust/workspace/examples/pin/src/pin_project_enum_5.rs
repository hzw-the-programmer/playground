use pin_project::pin_project;

/// Internal Map future
#[pin_project(project = MapProj, project_replace = MapProjReplace)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub enum Map<Fut, F> {
    Incomplete {
        #[pin]
        future: Fut,
        f: F,
    },
    Complete,
}

pub fn test() {}
