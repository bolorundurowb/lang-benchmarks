const fastify = require('fastify')({ logger: false });
const port = 7654;

fastify.get('/', async (request, reply) => {
    reply.send('Hello world!');
});

fastify.listen(port)
    .then(() => {
        console.log(`Listening on port ${port}`);
    });