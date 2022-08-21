/**
 * The html div element that we store the upcoming events summary
 */
let upcomingEventsContainer;

/**
 * The amount of secs to wait before updating the upcoming events summary
 * Currently it is set to 5 seconds
 */
let msecs = 5000;

/**
 * Calls the web server to get the rendered event summary HTML
 * @returns The rendered event summary HTML
 */
getRenderedUpcomingEventsSummaryData = async () => {
  try {
    let response = await fetch("/upcoming_events");
    return response.text();
  } catch (error) {
    console.log(error);
  }
};

/**
 * Sets up the initializes UpcomingEventsSummaryContainer variable
 * Must wait until windowed is loaded to use this function, otherwise
 * the target html element won't exist
 */
initializeUpcomingEventsContainer = () => {
  upcomingEventsContainer = document.getElementById("upcomingEventsContainer");
};

/**
 * Inserts the rendered event summary html on the summary page
 */
updateUpcomingEventsContainerData = async () => {
  upcomingEventsContainer.innerHTML =
    await getRenderedUpcomingEventsSummaryData();
};

/**
 * On window load, insert the event summary data , and repeat this
 * every ten seconds
 */
window.onload = () => {
  initializeUpcomingEventsContainer();
  updateUpcomingEventsContainerData();
  window.setInterval(() => {
    updateUpcomingEventsContainerData();
  }, msecs);
};
