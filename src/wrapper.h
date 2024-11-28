#include <swkbd/rpl_interface.h>
#include <vpadbase/base.h>
#include <mic/mic.h>
#include <padscore/wpad.h>
#include <padscore/kpad.h>
#include <nfc/nfc.h>
#include <wut_types.h>
#include <gfd.h>
#include <sys/select.h>
#include <sys/ioctl.h>
#include <sys/filio.h>
#include <sys/socket.h>
#include <sys/ioccom.h>
#include <gx2/tessellation.h>
#include <gx2/clear.h>
#include <gx2/context.h>
#include <gx2/enum.h>
#include <gx2/swap.h>
#include <gx2/display.h>
#include <gx2/state.h>
#include <gx2/temp.h>
#include <gx2/texture.h>
#include <gx2/shaders.h>
#include <gx2/debug.h>
#include <gx2/draw.h>
#include <gx2/surface.h>
#include <gx2/sampler.h>
#include <gx2/displaylist.h>
#include <gx2/utils.h>
#include <gx2/event.h>
#include <gx2/mem.h>
#include <gx2/aperture.h>
#include <gx2/registers.h>
#include <h264/decode.h>
#include <h264/stream.h>
#include <erreula/rpl_interface.h>
#include <nsyshid/hid.h>
#include <camera/camera.h>
#include <proc_ui/procui.h>
#include <proc_ui/memory.h>
#include <irc/cdc.h>
#include <irc/irc.h>
#include <netinet/tcp.h>
#include <netinet/in.h>
#include <nsyskbd/nsyskbd.h>
#include <poll.h>
#include <dmae/sync.h>
#include <dmae/mem.h>
#include <arpa/inet.h>
#include <wut_structsize.h>
#include <wut.h>
#include <nn/swkbd/swkbd_cpp.h>
#include <nn/nets2/somemopt.h>
#include <nn/nfp/amiibo_settings_cpp.h>
#include <nn/nfp/nfp_cpp.h>
#include <nn/acp/nn_acp_types.h>
#include <nn/acp/drcled_cpp.h>
#include <nn/acp/drcled_c.h>
#include <nn/acp/client.h>
#include <nn/acp/device.h>
#include <nn/acp/result.h>
#include <nn/acp/title.h>
#include <nn/acp/save.h>
#include <nn/hpad.h>
#include <nn/erreula.h>
#include <nn/acp.h>
#include <nn/sl/IStream.h>
#include <nn/sl/KillerNotificationSelector.h>
#include <nn/sl/IKillerNotificationAccessor.h>
#include <nn/sl/IAccountInfoAccessor.h>
#include <nn/sl/ITimeAccessor.h>
#include <nn/sl/ILaunchedTitleListAccessor.h>
#include <nn/sl/ITransferrer.h>
#include <nn/sl/ITitleListAccessor.h>
#include <nn/sl/IPreferentialTitleAccessor.h>
#include <nn/sl/KillerNotification.h>
#include <nn/sl/TitleListCache.h>
#include <nn/sl/DrcManager.h>
#include <nn/sl/ISettingAccessor.h>
#include <nn/sl/QuickStartApplicationSelector.h>
#include <nn/sl/ISerializer.h>
#include <nn/sl/DataCreator.h>
#include <nn/sl/IInstalledTitleListAccessor.h>
#include <nn/sl/IDiscCachedTitleAccessor.h>
#include <nn/sl/Condition.h>
#include <nn/sl/FileStream.h>
#include <nn/sl/KillerNotificationTransferRecordStream.h>
#include <nn/sl/LaunchInfoDatabase.h>
#include <nn/sl/IMetaInfoAccessor.h>
#include <nn/sl/IWhiteListAccessor.h>
#include <nn/sl/sl_cpp.h>
#include <nn/sl/ITitleIconCache.h>
#include <nn/sl/KillerNotificationTransferRecordManager.h>
#include <nn/sl/TitleIconCache.h>
#include <nn/sl/CacheManager.h>
#include <nn/sl/details/ILaunchedTitleListAccessoDetails.h>
#include <nn/sl/details/IStreamDetails.h>
#include <nn/sl/details/ITitleIconCacheDetails.h>
#include <nn/sl/details/ITitleListAccessorDetails.h>
#include <nn/sl/details/IInstalledTitleListAccessorDetails.h>
#include <nn/sl/details/IUpdatePackageAccessorDetails.h>
#include <nn/sl/details/IBlackListAccessorDetails.h>
#include <nn/sl/details/IIconInfoAccessorDetails.h>
#include <nn/sl/details/IKillerNotificationAccessorDetails.h>
#include <nn/sl/details/IPreferentialTitleAccessorDetails.h>
#include <nn/sl/details/IKillerNotificationTransferRecordManagerDetails.h>
#include <nn/sl/details/IDefaultTitleAccessorDetails.h>
#include <nn/sl/details/IMetaInfoAccessorDetails.h>
#include <nn/sl/details/ITimeAccessorDetails.h>
#include <nn/sl/details/ISerializerDetails.h>
#include <nn/sl/details/IAccountInfoAccessorDetails.h>
#include <nn/sl/details/ITransferrerDetails.h>
#include <nn/sl/details/ISettingAccessorDetails.h>
#include <nn/sl/details/IWhiteListAccessorDetails.h>
#include <nn/sl/details/IDiscCachedTitleAccessorDetails.h>
#include <nn/sl/IUpdatePackageAccessor.h>
#include <nn/sl/IDefaultTitleAccessor.h>
#include <nn/sl/IIconInfoAccessor.h>
#include <nn/sl/IBlackListAccessor.h>
#include <nn/spm.h>
#include <nn/ffl/miidata.h>
#include <nn/hpad/hpad.h>
#include <nn/hpad/beta.h>
#include <nn/ac.h>
#include <nn/dlp/Cafe.h>
#include <nn/nfp.h>
#include <nn/ac/ac_c.h>
#include <nn/ac/ac_cpp.h>
#include <nn/erreula/erreula_cpp.h>
#include <nn/ccr.h>
#include <nn/temp.h>
#include <nn/pdm/pdm_c.h>
#include <nn/pdm/pdm_cpp.h>
#include <nn/save/save.h>
#include <nn/sl.h>
#include <nn/nets2.h>
#include <nn/result.h>
#include <nn/act.h>
#include <nn/cmpt/cmpt.h>
#include <nn/dlp.h>
#include <nn/ccr/sys_caffeine.h>
#include <nn/ccr/sys.h>
#include <nn/spm/storage.h>
#include <nn/uds.h>
#include <nn/pdm.h>
#include <nn/fp/fp_cpp.h>
#include <nn/cmpt.h>
#include <nn/idb/idb_cpp.h>
#include <nn/idb/IDBReader.h>
#include <nn/swkbd.h>
#include <nn/uds/NodeInformation.h>
#include <nn/uds/Cafe.h>
#include <nn/uds/ScrambledLocalFriendCode.h>
#include <nn/temp/temp.h>
#include <nn/save.h>
#include <nn/idb.h>
#include <nn/cfg/CTR.h>
#include <nn/fp.h>
#include <nn/act/client_cpp.h>
#include <sysapp/launch.h>
#include <sysapp/title.h>
#include <sysapp/args.h>
#include <sysapp/switch.h>
#include <whb/log.h>
#include <whb/file.h>
#include <whb/proc.h>
#include <whb/log_console.h>
#include <whb/log_module.h>
#include <whb/sdcard.h>
#include <whb/libmanager.h>
#include <whb/log_cafe.h>
#include <whb/crash.h>
#include <whb/log_udp.h>
#include <whb/align.h>
#include <whb/gfx.h>
#include <tve/tve.h>
#include <tve/cec.h>
#include <sndcore2/drcvs.h>
#include <sndcore2/core.h>
#include <sndcore2/device.h>
#include <sndcore2/result.h>
#include <sndcore2/voice.h>
#include <gx2r/buffer.h>
#include <gx2r/resource.h>
#include <gx2r/draw.h>
#include <gx2r/surface.h>
#include <gx2r/displaylist.h>
#include <gx2r/mem.h>
#include <nsysccr/cdc.h>
#include <nsysccr/ccr.h>
#include <nsysccr/hid.h>
#include <nsysccr/cfg.h>
#include <nsysccr/irda.h>
#include <ntag/ntag.h>
#include <nsysnet/netconfig.h>
#include <nsysnet/_socket.h>
#include <nsysnet/_netdb.h>
// #include <nsysnet/socket.h>
#include <nsysnet/nssl.h>
#include <nsysnet/misc.h>
#include <avm/cec.h>
#include <avm/drc.h>
#include <avm/tv.h>
#include <avm/config.h>
#include <nsysuhs/uhs_usbspec.h>
#include <nsysuhs/uhs.h>
#include <netdb.h>
#include <wut_rplwrap.h>
#include <vpad/input.h>
#include <coreinit/semaphore.h>
#include <coreinit/threadqueue.h>
#include <coreinit/exception.h>
#include <coreinit/memlist.h>
#include <coreinit/fiber.h>
#include <coreinit/dynload.h>
#include <coreinit/stopwatch.h>
#include <coreinit/ipcbufpool.h>
#include <coreinit/rendezvous.h>
#include <coreinit/fastcondition.h>
#include <coreinit/kernel.h>
#include <coreinit/im.h>
#include <coreinit/exit.h>
#include <coreinit/energysaver.h>
#include <coreinit/scheduler.h>
#include <coreinit/filesystem.h>
// #include <coreinit/memblockheap.h>
#include <coreinit/context.h>
#include <coreinit/spinlock.h>
#include <coreinit/cache.h>
#include <coreinit/savedframe.h>
#include <coreinit/interrupts.h>
#include <coreinit/condition.h>
#include <coreinit/cosreport.h>
#include <coreinit/userconfig.h>
#include <coreinit/copydata.h>
#include <coreinit/alarm.h>
#include <coreinit/atomic.h>
// #include <coreinit/memunitheap.h>
#include <coreinit/core.h>
#include <coreinit/smd.h>
#include <coreinit/transition.h>
#include <coreinit/debug.h>
// #include <coreinit/memexpheap.h>
#include <coreinit/bsp.h>
#include <coreinit/ios.h>
#include <coreinit/foreground.h>
#include <coreinit/atomic64.h>
#include <coreinit/internal.h>
#include <coreinit/mutex.h>
#include <coreinit/taskqueue.h>
#include <coreinit/launch.h>
#include <coreinit/memorymap.h>
#include <coreinit/memdefaultheap.h>
#include <coreinit/filesystem_fsa.h>
#include <coreinit/stopwatchatomic.h>
#include <coreinit/performancemonitor.h>
#include <coreinit/messagequeue.h>
#include <coreinit/thread.h>
#include <coreinit/memfrmheap.h>
#include <coreinit/mcp.h>
#include <coreinit/codegen.h>
#include <coreinit/memory.h>
#include <coreinit/time.h>
#include <coreinit/title.h>
#include <coreinit/fastmutex.h>
#include <coreinit/coroutine.h>
#include <coreinit/event.h>
#include <coreinit/memheap.h>
#include <coreinit/screen.h>
#include <coreinit/systeminfo.h>

#include <malloc.h>
#include <errno.h>
