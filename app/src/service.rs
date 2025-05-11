use crate::repo::UserRepoContract;

struct UserService<R: UserRepoContract> {
    repo: R,
}

pub(crate) trait UserServiceContract {}
