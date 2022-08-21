/**
 * The html div element that we store the upcoming events summary
 */
let upcomingEventsSummaryContainer;

/**
 * The amount of secs to wait before updating the upcoming events summary
 * Currently it is set to 5 seconds
 */
let msecs = 5000;

/**
 * Calls the web server to get the rendered event summary HTML
 * @returns The rendered event summary HTML
 */
const getRenderedEventSummaryData = async () => {
  try {
    let response = await fetch("/summary_data");
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
const initializeUpcomingEventsSummaryContainer = () => {
  upcomingEventsSummaryContainer = document.getElementById(
    "upcomingEventsSummaryContainer"
  );
};

/**
 * Inserts the rendered event summary html on the summary page
 */
const updateUpcomingEventsSummaryContainerData = async () => {
  upcomingEventsSummaryContainer.innerHTML =
    await getRenderedEventSummaryData();
};

/**
 * On window load, insert the event summary data , and repeat this
 * every ten seconds
 */
window.onload = () => {
  initializeUpcomingEventsSummaryContainer();
  updateUpcomingEventsSummaryContainerData();
  window.setInterval(() => {
    updateUpcomingEventsSummaryContainerData();
  }, msecs);
};
