let getWinner = function(numberOfElves) {
  let presents = Array(numberOfElves).fill(1);

  let presentsList = presents.map((x, i) => {
    return {
      presents: 1,
      elf: i + 1
    };
  });
  let deleteEven = true;
  while (presentsList.length > 1) {
    // If starting position odd get rid of even, or vice versa
    let shouldDeleteEven = presentsList.length % 2 === 0;
    presentsList = presentsList.filter((x, i) => {
      if (deleteEven) {
        return i % 2 === 0;
      } else {
        return (i + 1) % 2 === 0;
      }
    });

    deleteEven = shouldDeleteEven;
  }
  console.log(
    `With ${numberOfElves} elves, elf ${
      presentsList[0].elf
    } gets all the presents`
  );
};

const input = 3014603;
getWinner(5);
getWinner(6);
getWinner(7);
getWinner(8);
getWinner(9);
getWinner(10);
getWinner(3014603);
