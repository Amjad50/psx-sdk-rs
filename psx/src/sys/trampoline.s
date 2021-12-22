// This file was automatically generated by build.rs
.set noreorder

.section .text.bios.file_open
.globl file_open
file_open:
    j 0xA0
    li $9, 0x00

.section .text.bios.file_seek
.globl file_seek
file_seek:
    j 0xA0
    li $9, 0x01

.section .text.bios.file_read
.globl file_read
file_read:
    j 0xA0
    li $9, 0x02

.section .text.bios.file_write
.globl file_write
file_write:
    j 0xA0
    li $9, 0x03

.section .text.bios.file_close
.globl file_close
file_close:
    j 0xA0
    li $9, 0x04

.section .text.bios.exit
.globl exit
exit:
    j 0xA0
    li $9, 0x06

.section .text.bios.save_state
.globl save_state
save_state:
    j 0xA0
    li $9, 0x13

.section .text.bios.restore_state
.globl restore_state
restore_state:
    j 0xA0
    li $9, 0x14

.section .text.bios.rand
.globl rand
rand:
    j 0xA0
    li $9, 0x2F

.section .text.bios.srand
.globl srand
srand:
    j 0xA0
    li $9, 0x30

.section .text.bios.malloc
.globl malloc
malloc:
    j 0xA0
    li $9, 0x33

.section .text.bios.free
.globl free
free:
    j 0xA0
    li $9, 0x34

.section .text.bios.calloc
.globl calloc
calloc:
    j 0xA0
    li $9, 0x37

.section .text.bios.realloc
.globl realloc
realloc:
    j 0xA0
    li $9, 0x38

.section .text.bios.init_heap
.globl init_heap
init_heap:
    j 0xA0
    li $9, 0x39

.section .text.bios.system_error_exit
.globl system_error_exit
system_error_exit:
    j 0xA0
    li $9, 0x3A

.section .text.bios.std_out_puts
.globl std_out_puts
std_out_puts:
    j 0xA0
    li $9, 0x3E

.section .text.bios.printf
.globl printf
printf:
    j 0xA0
    li $9, 0x3F

.section .text.bios.load_exe_header
.globl load_exe_header
load_exe_header:
    j 0xA0
    li $9, 0x41

.section .text.bios.load_exe_file
.globl load_exe_file
load_exe_file:
    j 0xA0
    li $9, 0x42

.section .text.bios.do_execute
.globl do_execute
do_execute:
    j 0xA0
    li $9, 0x43

.section .text.bios.flush_cache
.globl flush_cache
flush_cache:
    j 0xA0
    li $9, 0x44

.section .text.bios.gpu_send_dma
.globl gpu_send_dma
gpu_send_dma:
    j 0xA0
    li $9, 0x47

.section .text.bios.gp1_command
.globl gp1_command
gp1_command:
    j 0xA0
    li $9, 0x48

.section .text.bios.gp0_command
.globl gp0_command
gp0_command:
    j 0xA0
    li $9, 0x49

.section .text.bios.gp0_command_params
.globl gp0_command_params
gp0_command_params:
    j 0xA0
    li $9, 0x4A

.section .text.bios.gpu_get_status
.globl gpu_get_status
gpu_get_status:
    j 0xA0
    li $9, 0x4D

.section .text.bios.gpu_sync
.globl gpu_sync
gpu_sync:
    j 0xA0
    li $9, 0x4E

.section .text.bios.load_and_execute
.globl load_and_execute
load_and_execute:
    j 0xA0
    li $9, 0x51

.section .text.bios.cd_init
.globl cd_init
cd_init:
    j 0xA0
    li $9, 0x54

.section .text.bios.cd_remove
.globl cd_remove
cd_remove:
    j 0xA0
    li $9, 0x56

.section .text.bios.cd_async_get_status
.globl cd_async_get_status
cd_async_get_status:
    j 0xA0
    li $9, 0x7C

.section .text.bios.add_cdrom_device
.globl add_cdrom_device
add_cdrom_device:
    j 0xA0
    li $9, 0x96

.section .text.bios.set_memsize
.globl set_memsize
set_memsize:
    j 0xA0
    li $9, 0x9F

.section .text.bios.warm_boot
.globl warm_boot
warm_boot:
    j 0xA0
    li $9, 0xA0

.section .text.bios.cd_get_lbn
.globl cd_get_lbn
cd_get_lbn:
    j 0xA0
    li $9, 0xA4

.section .text.bios.cd_get_status
.globl cd_get_status
cd_get_status:
    j 0xA0
    li $9, 0xA6

.section .text.bios.get_system_info
.globl get_system_info
get_system_info:
    j 0xA0
    li $9, 0xB4

.section .text.bios.get_timer
.globl get_timer
get_timer:
    j 0xB0
    li $9, 0x03

.section .text.bios.enable_timer_irq
.globl enable_timer_irq
enable_timer_irq:
    j 0xB0
    li $9, 0x04

.section .text.bios.disable_timer_irq
.globl disable_timer_irq
disable_timer_irq:
    j 0xB0
    li $9, 0x05

.section .text.bios.restart_timer
.globl restart_timer
restart_timer:
    j 0xB0
    li $9, 0x06

.section .text.bios.deliver_event
.globl deliver_event
deliver_event:
    j 0xB0
    li $9, 0x07

.section .text.bios.open_event
.globl open_event
open_event:
    j 0xB0
    li $9, 0x08

.section .text.bios.close_event
.globl close_event
close_event:
    j 0xB0
    li $9, 0x09

.section .text.bios.wait_event
.globl wait_event
wait_event:
    j 0xB0
    li $9, 0x0A

.section .text.bios.test_event
.globl test_event
test_event:
    j 0xB0
    li $9, 0x0B

.section .text.bios.enable_event
.globl enable_event
enable_event:
    j 0xB0
    li $9, 0x0C

.section .text.bios.disable_event
.globl disable_event
disable_event:
    j 0xB0
    li $9, 0x0D

.section .text.bios.open_thread
.globl open_thread
open_thread:
    j 0xB0
    li $9, 0x0E

.section .text.bios.close_thread
.globl close_thread
close_thread:
    j 0xB0
    li $9, 0x0F

.section .text.bios.change_thread
.globl change_thread
change_thread:
    j 0xB0
    li $9, 0x10

.section .text.bios.init_pad
.globl init_pad
init_pad:
    j 0xB0
    li $9, 0x12

.section .text.bios.start_pad
.globl start_pad
start_pad:
    j 0xB0
    li $9, 0x13

.section .text.bios.stop_pad
.globl stop_pad
stop_pad:
    j 0xB0
    li $9, 0x14

.section .text.bios.set_default_exit_from_exception
.globl set_default_exit_from_exception
set_default_exit_from_exception:
    j 0xB0
    li $9, 0x18

.section .text.bios.undeliver_event
.globl undeliver_event
undeliver_event:
    j 0xB0
    li $9, 0x20

.section .text.bios.file_rename
.globl file_rename
file_rename:
    j 0xB0
    li $9, 0x44

.section .text.bios.file_delete
.globl file_delete
file_delete:
    j 0xB0
    li $9, 0x45

.section .text.bios.file_undelete
.globl file_undelete
file_undelete:
    j 0xB0
    li $9, 0x46

.section .text.bios.print_installed_devices
.globl print_installed_devices
print_installed_devices:
    j 0xB0
    li $9, 0x49

.section .text.bios.init_card
.globl init_card
init_card:
    j 0xB0
    li $9, 0x4A

.section .text.bios.start_card
.globl start_card
start_card:
    j 0xB0
    li $9, 0x4B

.section .text.bios.stop_card
.globl stop_card
stop_card:
    j 0xB0
    li $9, 0x4C

.section .text.bios.get_last_error
.globl get_last_error
get_last_error:
    j 0xB0
    li $9, 0x54

.section .text.bios.get_last_file_error
.globl get_last_file_error
get_last_file_error:
    j 0xB0
    li $9, 0x55

.section .text.bios.change_clear_pad
.globl change_clear_pad
change_clear_pad:
    j 0xB0
    li $9, 0x5B

.section .text.bios.change_clear_rcnt
.globl change_clear_rcnt
change_clear_rcnt:
    j 0xC0
    li $9, 0x0A

.section .text.bios.flush_std_in_out_put
.globl flush_std_in_out_put
flush_std_in_out_put:
    j 0xC0
    li $9, 0x13

.section .text.bios.enter_critical_section
.globl enter_critical_section
enter_critical_section:
    li $4, 0x01
    syscall 0x0
    jr $ra
    nop

.section .text.bios.exit_critical_section
.globl exit_critical_section
exit_critical_section:
    li $4, 0x02
    syscall 0x0
    jr $ra
    nop