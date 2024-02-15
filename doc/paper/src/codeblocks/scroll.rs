ScrollRight => {
    let amount = match self.hires {
        true => 4,
        false => 2 * 4,
    };

    self.display.iter_mut().for_each(|row| {
        row.rotate_right(amount);
        row[0..amount].fill(0);
    });
}
ScrollLeft => {
    let amount = match self.hires {
        true => 4,
        false => 2 * 4,
    };

    self.display.iter_mut().for_each(|row| {
        row.rotate_left(amount);
        row[DISPLAY_WIDTH - amount..].fill(0);
    });
}
ScrollDown { amount } => {
    let amount = match self.hires {
        true => amount,
        false => 2 * amount,
    };
    self.display.rotate_right(amount);
    self.display[0..amount].fill([0; DISPLAY_WIDTH]);
}
