import('./pkg')
  .then(m => m.greet('World!'))
  .catch(console.error);