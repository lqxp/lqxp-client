import {
  isPermissionGranted,
  requestPermission,
  sendNotification,
} from "@tauri-apps/plugin-notification";

export async function requestMessagingPermission(): Promise<boolean> {
  if (await isPermissionGranted()) {
    return true;
  }

  const permission = await requestPermission();
  return permission === "granted";
}

export async function notifyNewMessage(title: string, body: string): Promise<void> {
  if (!(await requestMessagingPermission())) {
    return;
  }

  sendNotification({ title, body });
}
