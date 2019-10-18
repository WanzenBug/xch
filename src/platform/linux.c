#include <fcntl.h>
#include <unistd.h>
#include <sys/syscall.h>

long linux_xch_syscall(const char* path1, const char* path2) {
    const int RENAME_EXCHANGE = 2;
    return syscall(SYS_renameat2, AT_FDCWD, path1, AT_FDCWD, path2, RENAME_EXCHANGE);
}
