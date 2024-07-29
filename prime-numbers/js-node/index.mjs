import { Worker } from 'worker_threads';
import {cpus} from 'os';

const numThreads = cpus().length;
const range = 2147483647;

function countPrimesInRange(start, end) {
  return new Promise((resolve, reject) => {
    const worker = new Worker('./worker.mjs', {
      workerData: { start, end }
    });

    worker.on('message', (count) => resolve(count));
    worker.on('error', (error) => reject(error));
    worker.on('exit', (code) => {
      if (code !== 0) {
        reject(new Error(`Worker stopped with exit code ${code}`));
      }
    });
  });
}

async function countAllPrimes(limit) {
  const segmentSize = Math.ceil(limit / numThreads);
  const promises = [];

  for (let i = 0; i < numThreads; i++) {
    const start = i * segmentSize;
    const end = Math.min((i + 1) * segmentSize - 1, limit);
    promises.push(countPrimesInRange(start, end));
  }

  const counts = await Promise.all(promises);
  return counts.reduce((acc, count) => acc + count, 0);
}

const main = async () => {
  const start = Date.now();
  const count = await countAllPrimes(range);
  const end = Date.now();
  console.log(`Total prime numbers between 0 and ${range}: ${count}`);
  console.log(`Time taken: ${end - start} ms`);
}

main().catch((error) => {
  console.error(`Error: ${error}`);
});
