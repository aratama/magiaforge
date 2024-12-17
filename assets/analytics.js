// Import the functions you need from the SDKs you need
import { initializeApp } from "https://www.gstatic.com/firebasejs/11.1.0/firebase-app.js";
import { getAnalytics } from "https://www.gstatic.com/firebasejs/11.1.0/firebase-analytics.js";
// TODO: Add SDKs for Firebase products that you want to use
// https://firebase.google.com/docs/web/setup#available-libraries

// Your web app's Firebase configuration
// For Firebase JS SDK v7.20.0 and later, measurementId is optional
const firebaseConfig = {
  apiKey: "AIzaSyCpI4k-G8OSNbduRrHTayJ5NCvuTHftrH0",
  authDomain: "magiaforge-f58d3.firebaseapp.com",
  projectId: "magiaforge-f58d3",
  storageBucket: "magiaforge-f58d3.firebasestorage.app",
  messagingSenderId: "190230318715",
  appId: "1:190230318715:web:5731bd3056843e2a05dcde",
  measurementId: "G-4E3FZPW8VP"
};

// Initialize Firebase
const app = initializeApp(firebaseConfig);
const analytics = getAnalytics(app);