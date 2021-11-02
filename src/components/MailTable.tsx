import React from "react";
import {
  Box,
  Checkbox,
  Paper,
  Table,
  TableBody,
  TableCell,
  TablePagination,
  TableRow,
  TableContainer,
  Typography,
} from "@mui/material";

import { MailTableHead, MailToolbar } from ".";
import { SelectAllRounded } from "@mui/icons-material";

const MailTable = (props: any) => {
  return (
    <Box
      sx={{
        width: "100%",
        height: { sm: "calc(100vh - 64px)", xs: "calc(100vh - 56px)" },
      }}
    >
      <Paper sx={{ width: "100%" }}>
        <MailToolbar numSelected={SelectAllRounded.length} />
        <TableContainer
          sx={{
            height: { sm: "calc(100vh - 180px)", xs: "calc(100vh - 160px)" },
          }}
        >
          <Table stickyHeader aria-label="sticky table" size="medium">
            <MailTableHead
              numSelected={selected.length}
              onSelectAllClick={handleSelectAllClick}
              rowCount={data.length}
              rowsPerPage={rowsPerPage}
              page={page}
              handleChangePage={handleChangePage}
              handleChangeRowsPerPage={handleChangeRowsPerPage}
            />
            <TableBody>
              {data
                .slice(page * rowsPerPage, page * rowsPerPage + rowsPerPage)
                .map((row, index) => {
                  const isItemSelected = isSelected(row.id);
                  const labelId = `enhanced-table-checkbox-${index}`;

                  return (
                    <TableRow
                      hover
                      role="checkbox"
                      aria-checked={isItemSelected}
                      tabIndex={-1}
                      key={row.id}
                      selected={isItemSelected}
                    >
                      <TableCell padding="checkbox">
                        <Checkbox
                          color="primary"
                          checked={isItemSelected}
                          inputProps={{
                            "aria-labelledby": labelId,
                          }}
                          onChange={(event) => handleChange(event, row.id)}
                        />
                      </TableCell>
                      <TableCell
                        component="th"
                        id={labelId}
                        scope="row"
                        padding="checkbox"
                        onClick={(event) => handleRowClick(row)}
                      >
                        <Typography
                          sx={{
                            textOverflow: "ellipsis",
                            overflow: "hidden",
                            width: { sm: 168, xs: 100 },
                          }}
                        >
                          {row.fromAddress}
                        </Typography>
                      </TableCell>
                      <TableCell onClick={(event) => handleRowClick(row)}>
                        <Typography
                          sx={{
                            width: { sm: 800, xs: 250 },
                            overflow: "hidden",
                            whiteSpace: "nowrap",
                            textOverflow: "ellipsis",
                          }}
                        >
                          {row.subject} - {row.body.substring(0, 90)}
                        </Typography>
                      </TableCell>
                    </TableRow>
                  );
                })}
              {emptyRows > 0 && (
                <TableRow
                  style={{
                    height: 53 * emptyRows,
                  }}
                >
                  <TableCell colSpan={6} />
                </TableRow>
              )}
            </TableBody>
          </Table>
        </TableContainer>
        <TablePagination
          rowsPerPageOptions={[5, 10, 25]}
          component="div"
          count={data.length}
          rowsPerPage={rowsPerPage}
          page={page}
          onPageChange={handleChangePage}
          onRowsPerPageChange={handleChangeRowsPerPage}
        />
      </Paper>
    </Box>
  );
};

export default MailTable;
