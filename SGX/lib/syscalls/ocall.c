#include <sys/types.h>
#include<stdio.h>
#include <unistd.h>
u_int64_t ocall_syscall_write(size_t fd, const char *buf, size_t count){
	int res=write(fd,buf,count);
	if (res==-1){printf("write error");}
	return res;
}
