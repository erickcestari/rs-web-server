let count = 0;
let counter = document.querySelector('.counter');

function updateCounter() {
  counter.innerHTML = count;
}

document.querySelector('.increment').addEventListener('click', () => {
  count++;
  updateCounter();
});

document.querySelector('.decrement').addEventListener('click', () => {
  count--;
  updateCounter();
});
