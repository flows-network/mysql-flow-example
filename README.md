# A flow function with a MySQL backend for CRUD operations

## How it works

This example used webhook integration to create the CRUD API for the `orders` table.

## Deploy

1. Deploy the webhook from the template.
2. Add your database credentials to the webhook.
    - Set `DATABASE_URL` to `mysql://<USER>:<PASS>@<HOST>:<PORT>/<DB>"` in Flows.network settings.

## Example

### Initialize the database

```bash
$ curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=init' | jq .
{
  "status": "success"
}
```

### Query all orders

```bash
$ curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=queryAll' | jq .
{
  "data": [
    {
      "amount": 56,
      "order_id": 1,
      "production_id": 12,
      "quantity": 2,
      "shipping": 15,
      "shipping_address": "Mataderos 2312",
      "tax": 2
    },
    {
      "amount": 256,
      "order_id": 2,
      "production_id": 15,
      "quantity": 3,
      "shipping": 30,
      "shipping_address": "1234 NW Bobcat",
      "tax": 16
    },
    {
      "amount": 536,
      "order_id": 3,
      "production_id": 11,
      "quantity": 5,
      "shipping": 50,
      "shipping_address": "20 Havelock",
      "tax": 24
    },
    {
      "amount": 126,
      "order_id": 4,
      "production_id": 8,
      "quantity": 8,
      "shipping": 20,
      "shipping_address": "224 Pandan Loop",
      "tax": 12
    },
    {
      "amount": 46,
      "order_id": 5,
      "production_id": 24,
      "quantity": 1,
      "shipping": 10,
      "shipping_address": "No.10 Jalan Besar",
      "tax": 2
    }
  ],
  "status": "success"
}
```

### Delete an order

```bash
[~] ➟  curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=deleteById&order_id=5' | jq .
{
  "status": "success"
}
```

Check the order was deleted.

```bash
$ curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=queryAll' | jq .
{
  "data": [
    {
      "amount": 56,
      "order_id": 1,
      "production_id": 12,
      "quantity": 2,
      "shipping": 15,
      "shipping_address": "Mataderos 2312",
      "tax": 2
    },
    {
      "amount": 256,
      "order_id": 2,
      "production_id": 15,
      "quantity": 3,
      "shipping": 30,
      "shipping_address": "1234 NW Bobcat",
      "tax": 16
    },
    {
      "amount": 536,
      "order_id": 3,
      "production_id": 11,
      "quantity": 5,
      "shipping": 50,
      "shipping_address": "20 Havelock",
      "tax": 24
    },
    {
      "amount": 126,
      "order_id": 4,
      "production_id": 8,
      "quantity": 8,
      "shipping": 20,
      "shipping_address": "224 Pandan Loop",
      "tax": 12
    }
  ],
  "status": "success"
}
```

### Query the order by order_id

```bash
$ curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=queryById&order_id=3' | jq .
{
  "data": [
    {
      "amount": 536,
      "order_id": 3,
      "production_id": 11,
      "quantity": 5,
      "shipping": 50,
      "shipping_address": "20 Havelock",
      "tax": 24
    }
  ],
  "status": "success"
}
```

### Update the shipping address

```bash
$ curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=updateAddressById&order_id=3&shipping_address=NewAddress' | jq .
{
  "status": "success"
}
```

Check the order information was updated.

```bash
$ curl -s 'https://code.flows.network/webhook/tv0CFnwNMwnBzs9QqRKw?action=queryById&order_id=3' | jq .
{
  "data": [
    {
      "amount": 536,
      "order_id": 3,
      "production_id": 11,
      "quantity": 5,
      "shipping": 50,
      "shipping_address": "NewAddress",
      "tax": 24
    }
  ],
  "status": "success"
}
```
