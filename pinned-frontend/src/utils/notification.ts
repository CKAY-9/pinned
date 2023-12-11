const createNotification = (
  message: string = "No message provided", 
  time_on_screen_ms: number = 5000
) => {
  const notifications_container = document.getElementById("notifications_main"); 
  if (notifications_container === null) {
    console.error("Failed to get notifications container!");
    return;
  }

  const notification_id: number = new Date().getTime();
  notifications_container.innerHTML += `
    <div className="notification" id="${notification_id}"}>${message}</div>
  `;
  const notification = document.getElementById(`${notification_id}`);
  setTimeout(() => {
    if (notification === null) {
      return;
    }
    notification.remove();
  }, time_on_screen_ms);
}
