import { homework_1_backend } from "../../declarations/homework_1_backend";
const date = new Date();

let hours = date.getHours();
let minutes = date.getMinutes();
let day = date.getDate();
let month = date.getMonth() + 1;
let year = date.getFullYear();
document.getElementById("datetime").innerText = `${hours}:${minutes} , ${day}-${month}-${year}`;


document.querySelector("form").addEventListener("submit", async (e) => {
  e.preventDefault();
  const button = e.target.querySelector("button");
  const name = document.getElementById("name").value.toString();

  button.setAttribute("disabled", true);
  get_weather_data(name);
  button.removeAttribute("disabled");

  return false;
});

async function get_weather_data(name){
  const api_data = await homework_1_backend.get_weather_by_city_name(name);
  document.getElementById("city_name").innerText = name

  // let obj = JSON.parse(JSON.stringify(api_data));
  // let error = api_data["Err"];
  let data = api_data["Ok"];
  if (data == undefined) {
    document.getElementById("temprature").innerText = "-"
    document.getElementById("weather_condition").innerText = "-"
    document.getElementById("humidity").innerText = "-"
    document.getElementById("wind").innerText = "-"
    document.getElementById("wind_direction").innerText = "-"

  }
  else {
    console.log(data);
    document.getElementById("temprature").innerText = `${data["main"].temp}\xB0C `
    document.getElementById("weather_condition").innerText = data["weather"][0]["description"]
    document.getElementById("humidity").innerText = data["main"].humidity;
    document.getElementById("wind").innerText = `${data["wind"].speed} km/h`;
    document.getElementById("wind_direction").innerHTML = `${data["wind"].deg}&deg;`

    var weather_icon_id = data["weather"][0]["id"];
    var weather_icon_result = Math.trunc(weather_icon_id/100);
    if(weather_icon_id !=800 && weather_icon_result==8){
      weather_icon_result="8x"
    }
    console.log(weather_icon_result);
    document.getElementById("weather_icon").src = `${weather_icon_result}.png`
  }

  const date = new Date();

  let hours = date.getHours();
  let minutes = date.getMinutes();
  let day = date.getDate();
  let month = date.getMonth() + 1;
  let year = date.getFullYear();
  document.getElementById("datetime").innerText = `${hours}:${minutes} , ${day}-${month}-${year}`;

}


document.getElementById("london").onclick = function(){
  get_weather_data("London")
}
document.getElementById("istanbul").onclick = function(){
  get_weather_data("Istanbul")
}
document.getElementById("california").onclick = function(){
  get_weather_data("California")
}
document.getElementById("sydney").onclick = function(){
  get_weather_data("Sydney")
}
