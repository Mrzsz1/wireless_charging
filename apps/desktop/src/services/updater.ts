import { relaunch } from '@tauri-apps/plugin-process'
import { check, type DownloadEvent } from '@tauri-apps/plugin-updater'
import { getVersion } from '@tauri-apps/api/app'

export type UpdateState =
  | { status: 'current'; message: string }
  | { status: 'available'; message: string; version: string }
  | { status: 'downloading'; message: string; version: string; downloaded: number; total?: number }

export type AppReleaseInfo = {
  version: string
  channel: string
}

export async function getAppReleaseInfo(): Promise<AppReleaseInfo> {
  return {
    version: await getVersion(),
    channel: import.meta.env.VITE_UPDATE_CHANNEL?.trim() || 'stable',
  }
}

export async function checkAndInstallUpdate(
  onState: (state: UpdateState) => void,
): Promise<void> {
  const update = await check()
  if (!update) {
    onState({ status: 'current', message: '当前已是最新版本' })
    return
  }
  onState({ status: 'available', message: `发现版本 ${update.version}`, version: update.version })
  let downloaded = 0
  let total: number | undefined
  await update.downloadAndInstall((event: DownloadEvent) => {
    if (event.event === 'Started') total = event.data.contentLength ?? undefined
    if (event.event === 'Progress') downloaded += event.data.chunkLength
    if (event.event !== 'Finished') {
      onState({
        status: 'downloading',
        message: total ? `正在下载 ${Math.min(100, Math.round(downloaded / total * 100))}%` : `已下载 ${downloaded} 字节`,
        version: update.version,
        downloaded,
        total,
      })
    }
  })
  await relaunch()
}
