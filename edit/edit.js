const schema =
{
  "$id": "http://example.com/example.json",
  "type": "object",
  "definitions": {},
  "$schema": "http://json-schema.org/draft-07/schema#",
  "properties": {
    "header": {
      "$id": "/properties/header",
      "type": "object",
      "properties": {
        "navbar": {
          "$id": "/properties/header/properties/navbar",
          "type": "array",
          "items": {
            "$id": "/properties/header/properties/navbar/items",
            "type": "object",
            "properties": {
              "link": {
                "$id": "/properties/header/properties/navbar/items/properties/link",
                "type": "string",
                "title": "The Link Schema ",
                "default": "",
                "examples": [
                  "/find-club/"
                ]
              },
              "text": {
                "$id": "/properties/header/properties/navbar/items/properties/text",
                "type": "string",
                "title": "The Text Schema ",
                "default": "",
                "examples": [
                  "Find A Club"
                ]
              }
            }
          }
        },
        "jumbotron": {
          "$id": "/properties/header/properties/jumbotron",
          "type": "boolean",
          "title": "The Jumbotron Schema ",
          "default": false,
          "examples": [
            false
          ]
        }
      }
    },
    "footer": {
      "$id": "/properties/footer",
      "type": "boolean",
      "title": "The Footer Schema ",
      "default": true,
      "examples": [
        true
      ]
    }
  }
};
const BrutusinForms = brutusin["json-forms"];
const bf = BrutusinForms.create(schema);

const container = document.getElementById("container");
bf.render(container);

const eye = document.querySelector("iframe");

const update = document.getElementById("update");
update.addEventListener("click", refresh);

async function refresh() {
  if (bf.validate()) {
    const data = bf.getData();
    let id = await postData("../update", data);
    eye.src = "../preview/" + id;
  }
}

async function postData(url, data) {
  return fetch(url, {
    body: JSON.stringify(data),
    headers: {
      'content-type': 'application/json'
    },
    method: 'POST',
  })
  .then(response => response.json())
}
